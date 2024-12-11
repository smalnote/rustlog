use std::{
    alloc::{self, Layout},
    arch::x86_64 as x86,
    borrow::Borrow,
    hash::{BuildHasher, Hash},
    marker::PhantomData,
    mem,
    num::NonZeroU16,
    ops::Deref,
    ptr::{self, NonNull},
};

pub struct SwissTable<K, V> {
    words: NonNull<u8>,
    groups: NonNull<(K, V)>,
    /// Number of groups set to power of 2, mask set to n^2 - 1,
    /// index & mask is equivalent to index % n ^ 2
    mask: u64,
    cap: usize,
    len: usize,
    hash_builder: std::hash::RandomState,
}

impl<K, V> Default for SwissTable<K, V>
where
    K: Hash + Eq,
{
    fn default() -> Self {
        Self::new()
    }
}

pub(crate) const GROUP_SIZE: usize = 16;
pub(crate) const NUMBER_GROUP: usize = 8; // 2^3

impl<K, V> SwissTable<K, V>
where
    K: Hash + Eq,
{
    /// Creates a fixed capacity of SwissTable.
    pub fn new() -> Self {
        let cap = NUMBER_GROUP * GROUP_SIZE;
        // We assume SIMD supports 128-bit (16bytes) operation.
        // Hence, one control byte for a entry, 16 elements per group.
        // For simplicity, we create a fixed capacity of table with
        // 8 (2^3) groups, 8 * 16 entry at most.
        let words = Self::alloc_group_words(cap);
        let groups = Self::alloc_groups(cap);
        Self {
            words,
            groups,
            mask: (NUMBER_GROUP - 1) as u64,
            cap,
            len: 0,
            hash_builder: Default::default(),
        }
    }

    /// Creates control bytes for entries, one byte for an entry.
    /// All control byte are initially empty.
    fn alloc_group_words(cap: usize) -> NonNull<u8> {
        // group words buffer is array of byte, one byte for a entry
        let layout = alloc::Layout::array::<u8>(cap).unwrap();
        unsafe {
            let ptr = alloc::alloc(layout);
            std::ptr::write_bytes(ptr, Tag::EMPTY, layout.size());
            NonNull::new(ptr).unwrap()
        }
    }

    fn alloc_groups(cap: usize) -> NonNull<(K, V)> {
        // groups buffer is array of (K, V) tuples
        let layout = alloc::Layout::array::<(K, V)>(cap).unwrap();
        unsafe { NonNull::new(alloc::alloc(layout) as *mut (K, V)).unwrap() }
    }

    pub fn cap(&self) -> usize {
        self.cap
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn hash<Q>(&self, key: &Q) -> (usize, Tag)
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        let h = self.hash_builder.hash_one(key);
        // Lower 7-bit as H2
        let h2 = (h & 0x01_111_111) as u8;
        // Higher 57-bit as H1
        let h1 = (h >> 7) as usize;
        (h1, Tag(h2))
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.len == self.cap {
            panic!("too many entries")
        }
        let (h1, h2) = self.hash(&key);
        match self.find_entry_or_insert_slot(h1, h2, &key) {
            Ok(mut bucket) => {
                let dst = unsafe { &mut bucket.ptr.as_mut().1 };
                let replaced = mem::replace(dst, value);
                Some(replaced)
            }
            Err(slot) => {
                unsafe {
                    self.len += 1;
                    ptr::write(self.words.add(slot.index).as_ptr(), h2.0);
                    ptr::write(self.groups.add(slot.index).as_ptr(), (key, value));
                }
                None
            }
        }
    }

    fn find_entry_or_insert_slot(
        &mut self,
        h1: usize,
        h2: Tag,
        key: &K,
    ) -> Result<Bucket<(K, V)>, InsertSlot> {
        // * Caller should ensure at least one available slot, or probing will
        // be a infinite loop
        let mut probe_seq = ProbeSeq::from(h1, self.mask as usize);
        let mut not_found = false;
        loop {
            let offset = probe_seq.group_index * GROUP_SIZE;
            // First: try find a entry for key
            let word: GroupWord = unsafe { self.words.add(offset).into() };
            let group: Group<K, V> = unsafe { self.groups.add(offset).into() };
            for entry_index in word.match_tag(&h2) {
                if group[entry_index].0.borrow().equivalent(key) {
                    return Ok(Bucket {
                        ptr: unsafe { self.groups.add(offset + entry_index) },
                    });
                }
            }

            if let Some(available_slot) = word.match_empty_or_deleted().lowest_set_bit() {
                if word.match_empty().any_bit_set() || not_found {
                    // If there is a empty slot, we stop probing, and use a empty or deleted slot.
                    // Or we have probed all group and not empty slot found, use the deleted slot.
                    return Err(InsertSlot {
                        index: offset + available_slot,
                    });
                }
            }

            // If group is all full or deleted, we should keep probing
            if probe_seq.move_next().is_none() {
                // All groups probed and no empty slot, restart probing a deleted slot
                probe_seq = ProbeSeq::from(h1, self.mask as usize);
                not_found = true;
            }
        }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        if self.is_empty() {
            return None;
        }
        let (h1, h2) = self.hash::<Q>(key);
        let mut probe_seq = ProbeSeq::from(h1, self.mask as usize);
        loop {
            let offset = probe_seq.group_index * GROUP_SIZE;
            // First: try find a entry for key
            let word: GroupWord = unsafe { self.words.add(offset).into() };
            let group: Group<K, V> = unsafe { self.groups.add(offset).into() };
            for entry_index in word.match_tag(&h2) {
                if key.equivalent(&group[entry_index].0) {
                    return unsafe {
                        let ptr = self.groups.add(offset + entry_index);
                        Some(&ptr.as_ref().1)
                    };
                }
            }

            if word.match_empty().any_bit_set() {
                return None;
            }
            probe_seq.move_next()?;
        }
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        if self.len == 0 {
            return None;
        }
        let (h1, h2) = self.hash(key);
        let mut probe_seq = ProbeSeq::from(h1, self.mask as usize);
        loop {
            let offset = probe_seq.group_index * GROUP_SIZE;
            // First: try find a entry for key
            let word: GroupWord = unsafe { self.words.add(offset).into() };
            let group: Group<K, V> = unsafe { self.groups.add(offset).into() };
            for entry_index in word.match_tag(&h2) {
                if key.equivalent(&group[entry_index].0) {
                    self.len -= 1;
                    let (_, value) = unsafe {
                        ptr::write(self.words.add(offset + entry_index).as_ptr(), Tag::DELETED);
                        ptr::read(self.groups.add(offset + entry_index).as_ptr())
                    };
                    return Some(value);
                }
            }

            if word.match_empty().any_bit_set() {
                return None;
            }
            probe_seq.move_next()?;
        }
    }
}

impl<K, V> SwissTable<K, V> {
    fn drain(&mut self) -> Drain<'_, K, V> {
        let len = self.len;
        self.len = 0;
        Drain {
            index: 0,
            len,
            group_word: self.words,
            group: self.groups,
            bit_mask_iter: GroupWord::from(self.words).match_full().into_iter(),
            _table: PhantomData,
        }
    }
}

impl<K, V> Drop for SwissTable<K, V> {
    fn drop(&mut self) {
        for _ in self.drain() {}
        let words_layout = Layout::array::<u8>(NUMBER_GROUP * GROUP_SIZE).unwrap();
        let groups_layout = Layout::array::<(K, V)>(NUMBER_GROUP * GROUP_SIZE).unwrap();
        unsafe {
            alloc::dealloc(self.words.as_ptr() as *mut _, words_layout);
            alloc::dealloc(self.groups.as_ptr() as *mut _, groups_layout);
        }
    }
}

/// Key equivalence trait.
///
/// This trait allows hash table lookup to be customized. It has one blanket
/// implementation that uses the regular solution with `Borrow` and `Eq`, just
/// like `HashMap` does, so that you can pass `&str` to lookup into a map with
/// `String` keys and so on.
///
/// # Contract
///
/// The implementor **must** hash like `K`, if it is hashable.
pub trait Equivalent<K: ?Sized> {
    /// Compare self to `key` and return `true` if they are equal.
    fn equivalent(&self, key: &K) -> bool;
}

impl<Q: ?Sized, K: ?Sized> Equivalent<K> for Q
where
    Q: Eq,
    K: Borrow<Q>,
{
    #[inline]
    fn equivalent(&self, key: &K) -> bool {
        PartialEq::eq(self, key.borrow())
    }
}

struct Drain<'a, K, V> {
    index: usize,
    len: usize,
    group_word: NonNull<u8>,
    group: NonNull<(K, V)>,
    bit_mask_iter: BitMaskIter,
    // Logically Drain borrow a mutable table, without this marker, caller is
    // able to mutate the table while draining, result in data race on group
    // and group_word.
    _table: PhantomData<&'a mut SwissTable<K, V>>,
}

impl<K, V> Iterator for Drain<'_, K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else if let Some(slot_index) = self.bit_mask_iter.next() {
            self.len -= 1;

            //* Safe: we are iterating group word linearly, and the table will be empty eventually
            unsafe { ptr::write(self.group_word.add(slot_index).as_ptr(), Tag::EMPTY) };

            Some(unsafe { ptr::read(self.group.add(slot_index).as_ptr()) })
        } else if self.index < NUMBER_GROUP - 1 {
            self.index += 1;
            unsafe {
                self.group_word = self.group_word.add(GROUP_SIZE);
                self.group = self.group.add(GROUP_SIZE);
                self.bit_mask_iter = GroupWord::from(self.group_word).match_full().into_iter();
            }
            self.next()
        } else {
            None
        }
    }
}

impl<K, V> Drop for Drain<'_, K, V> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

struct ProbeSeq {
    /// Current probing group index
    group_index: usize,
    /// Number of group -1 (2^n-1), for mod 2^n
    mask: usize,
    /// Start index for check overlapping
    start_index: usize,
}

impl ProbeSeq {
    fn from(h1: usize, mask: usize) -> ProbeSeq {
        let start_index = h1 & mask;
        Self {
            group_index: start_index,
            mask,
            start_index,
        }
    }
    fn move_next(&mut self) -> Option<usize> {
        // Simply do linear probing
        self.group_index = (self.group_index + 1) & self.mask;
        if self.group_index == self.start_index {
            None
        } else {
            Some(self.group_index)
        }
    }
}

/// A reference to a SwissTable bucket containing a `T`.
struct Bucket<T> {
    ptr: NonNull<T>,
}

/// A reference to an empty bucket into which an can be inserted.
struct InsertSlot {
    index: usize,
}

/// GroupWord is a 16-byte for a group a entry.
struct GroupWord {
    m128i: x86::__m128i,
}

impl From<NonNull<u8>> for GroupWord {
    fn from(value: NonNull<u8>) -> Self {
        Self {
            m128i: unsafe { x86::_mm_load_si128(value.as_ptr() as *const _) },
        }
    }
}

impl GroupWord {
    fn match_tag(&self, tag: &Tag) -> BitMask {
        unsafe {
            let cmp_ret = x86::_mm_cmpeq_epi8(self.m128i, x86::_mm_set1_epi8(tag.0 as i8));
            BitMask(x86::_mm_movemask_epi8(cmp_ret) as u16)
        }
    }

    fn match_empty_or_deleted(&self) -> BitMask {
        // empty or deleted has most significant bit set to 1
        unsafe { BitMask(x86::_mm_movemask_epi8(self.m128i) as u16) }
    }

    fn match_empty(&self) -> BitMask {
        unsafe {
            let cmp_ret = x86::_mm_cmpeq_epi8(self.m128i, x86::_mm_set1_epi8(Tag::EMPTY as i8));
            BitMask(x86::_mm_movemask_epi8(cmp_ret) as u16)
        }
    }

    fn match_full(&self) -> BitMask {
        self.match_empty_or_deleted().invert()
    }
}

/// BitMask donates 16-tag group match result.
struct BitMask(u16);

impl BitMask {
    fn lowest_set_bit(&self) -> Option<usize> {
        NonZeroU16::new(self.0).map(|non_zero| non_zero.trailing_zeros() as usize)
    }

    fn remove_lowest_bit(&self) -> Self {
        // If right most bit is zero, -1 just make it zero, ok
        // If not, it is in the form of 0b?10..0, with k trailing 0
        // subtract 1, get 0b?01..1
        // 0b?10..0
        // 0b?01..1 &
        // 0b?00..0 exactly set lowest bit to 0
        Self(self.0 & (self.0 - 1))
    }

    fn any_bit_set(&self) -> bool {
        self.0 != 0
    }

    fn invert(&self) -> Self {
        Self(self.0 ^ 0xFFFF)
    }
}

impl IntoIterator for BitMask {
    type Item = usize;
    type IntoIter = BitMaskIter;

    fn into_iter(self) -> Self::IntoIter {
        BitMaskIter(self)
    }
}

struct BitMaskIter(BitMask);

/// Iterating set bits of mask.
impl Iterator for BitMaskIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let bit = self.0.lowest_set_bit()?;
        self.0 = self.0.remove_lowest_bit();
        Some(bit)
    }
}

/// Tag is a GroupWord control byte for a slot in group.
#[derive(Clone, Copy)]
struct Tag(u8);

impl Tag {
    const EMPTY: u8 = 0b10_000_000;
    const DELETED: u8 = 0b11_111_110;
}

struct Group<K, V> {
    ptr: NonNull<(K, V)>,
}

impl<K, V> From<NonNull<(K, V)>> for Group<K, V> {
    fn from(value: NonNull<(K, V)>) -> Self {
        Self { ptr: value }
    }
}

impl<K, V> Deref for Group<K, V> {
    type Target = [(K, V)];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr() as *const _, GROUP_SIZE) }
    }
}

// x86_64 SIMD as example
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swiss_table() {
        let mut table = SwissTable::new();

        for i in 0..128 {
            table.insert(i, i);
        }

        for i in 64..128 {
            let removed = table.remove(&i);
            assert_eq!(removed.unwrap(), i);
        }

        for i in 0..64 {
            assert_eq!(table.get(&i).unwrap(), &i);
        }

        for i in 64..128 {
            assert_eq!(table.get(&i), None);
        }

        for i in 256..320 {
            table.insert(i, i);
        }

        for i in 256..320 {
            assert_eq!(table.get(&i).unwrap(), &i);
        }

        for i in 0..320 {
            let removed = table.remove(&i);
            match i {
                0..64 | 256..320 => assert_eq!(removed.unwrap(), i),
                _ => assert!(removed.is_none()),
            }
        }

        assert_eq!(table.len(), 0);
    }

    #[test]
    fn test_swiss_table_drop() {
        let mut table = SwissTable::new();

        assert!(table.drain().next().is_none());

        for i in 0..32 {
            table.insert(i, i.to_string());
        }

        for i in 0..32 {
            assert_eq!(table.get(&i).unwrap(), &i.to_string());
        }

        let all_entries: Vec<(i32, String)> = table.drain().collect();
        assert_eq!(all_entries.len(), 32);

        for i in 16..96 {
            table.insert(i, i.to_string());
        }

        for i in 16..96 {
            assert_eq!(table.get(&i).unwrap(), &i.to_string());
        }

        assert_eq!(table.len(), 80);
    }

    #[test]
    fn test_create_empty_swiss_table() {
        let table: SwissTable<usize, usize> = Default::default();
        let default_cap = GROUP_SIZE * NUMBER_GROUP;
        assert_eq!(table.cap(), default_cap);
        assert_eq!(table.len(), 0);
        assert!(table.is_empty());
    }

    use std::{
        arch::x86_64::{_mm_load_si128, _mm_movemask_epi8},
        collections::HashMap,
    };

    #[test]
    fn test_simd() {
        // control byte
        const EMPTY: u8 = 0b10_000_000;
        const DELETED: u8 = 0b11_111_110;
        const FULL: u8 = 0b01_000_011;
        unsafe {
            // from lower bit higher bit
            let tags: [u8; 16] = [
                EMPTY, EMPTY, EMPTY, DELETED, FULL, DELETED, FULL, EMPTY, FULL, DELETED, FULL,
                EMPTY, DELETED, EMPTY, FULL, FULL,
            ];
            // since we only use tags ptr later, visit it make sure it is initialized
            for _ in tags {}

            // get slice start pointer, not fat array pointer
            let ptr = &tags[..];
            let ptr = ptr.as_ptr();
            // construct a control of 16 bytes from raw bytes
            let group_word = _mm_load_si128(ptr as *const _);
            // SIMD instruction, match EMPTY or DELETED byte
            // since EMPTY or DELETE byte set highest bit to 1
            // use SIMD to collect bytes with highest bit set to 1
            let mask = _mm_movemask_epi8(group_word) as u16;
            // bits with 1 are empty or delete
            assert_eq!(mask, 0b0011_1010_1010_1111);
        }
    }

    #[test]
    fn test_hash_brown_map() {
        // use hashbrown for debugging, since std lib jump into assembly code at some points
        let mut m: hashbrown::HashMap<i32, String> = hashbrown::HashMap::with_capacity(7);

        m.insert(10, "alpha".to_string());
        m.insert(20, "beta".to_string());
        m.insert(30, "gamma".to_string());

        m.remove(&20);
        m.remove(&10);

        m.get(&10);

        dbg!(m);
    }

    #[test]
    fn test_std_hash_map_with_debug() {
        let mut m = HashMap::with_capacity(16);

        m.insert(10, 10);
        m.insert(20, 20);
        m.insert(30, 30);

        m.remove(&10);
        m.remove(&20);

        dbg!(m);
    }

    #[test]
    fn validate_simd_features() {
        #[cfg(all(
            target_feature = "sse2",
            any(target_arch = "x86", target_arch = "x86_64"),
            not(miri)
        ))]
        println!("SIMD sse2 is enabled");
    }
}
