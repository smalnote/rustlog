use std::{
    alloc::{self, Layout},
    arch::x86_64 as x86,
    borrow::Borrow,
    hash::{BuildHasher, Hash},
    mem,
    num::NonZeroU16,
    ops::Deref,
    ptr::{self, NonNull},
};

use siphasher::sip::SipHasher13;

pub struct SwissTable<K, V> {
    metas: NonNull<u8>,
    groups: NonNull<(K, V)>,
    /// Number of groups set to power of 2, mask set to n^2 - 1,
    /// index & mask is equivalent to index % n ^ 2
    mask: u64,
    cap: usize,
    len: usize,
    hahser_builder: DefaultHashBuilder,
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
        let metas = Self::alloc_metas(cap);
        let groups = Self::alloc_groups(cap);
        Self {
            metas,
            groups,
            mask: (NUMBER_GROUP - 1) as u64,
            cap,
            len: 0,
            hahser_builder: Default::default(),
        }
    }

    /// Creates control bytes for entries, one byte for an entry.
    /// All control byte are initially empty.
    fn alloc_metas(cap: usize) -> NonNull<u8> {
        // metadatas buffer is array of byte, one byte for a entry
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

    fn hash<Q>(&self, key: &Q) -> (u64, Tag)
    where
        Q: Hash + Eq,
        K: Borrow<Q>,
    {
        let h = self.hahser_builder.hash_one(key);
        // Lower 7-bit as H2
        let h2 = (h & 0x01_111_111) as u8;
        // Higher 57-bit as H1
        let h1 = h >> 7;
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
                    ptr::write(self.metas.add(slot.index).as_ptr(), h2.0);
                    ptr::write(self.groups.add(slot.index).as_ptr(), (key, value));
                }
                None
            }
        }
    }

    fn find_entry_or_insert_slot(
        &mut self,
        h1: u64,
        h2: Tag,
        key: &K,
    ) -> Result<Bucket<(K, V)>, InsertSlot> {
        // TODO: use ProbSeq instead raw linear probing
        let start_index = h1 & self.mask;
        let mut index = start_index;
        loop {
            let offset = index as usize * GROUP_SIZE;
            // First: try find a entry for key
            let meta: Metadata = unsafe { self.metas.add(offset).into() };
            let group: Group<K, V> = unsafe { self.groups.add(offset).into() };
            for entry_index in meta.match_tag(&h2) {
                if group[entry_index].0 == *key {
                    return Ok(Bucket {
                        ptr: unsafe { self.groups.add(offset + entry_index) },
                    });
                }
            }

            if let Some(slot_index) = meta.match_empty_or_deleted().into_iter().next() {
                return Err(InsertSlot {
                    index: offset + slot_index,
                });
            }
            index = (index + 1) & self.mask; // we simply do linear probing
            if index == start_index {
                // all group probed
                panic!("too many entries");
            }
        }
    }

    // TODO: K -> &Q, remove K:Copy
    pub fn get(&self, key: &K) -> Option<&V>
    where
        K: Copy,
    {
        if self.len == 0 {
            return None;
        }
        // TODO: use ProbSeq instead raw linear probing
        let (h1, h2) = self.hash(key);
        let start_index = h1 & self.mask;
        let mut index = start_index;
        loop {
            let offset = index as usize * GROUP_SIZE;
            // First: try find a entry for key
            let meta: Metadata = unsafe { self.metas.add(offset).into() };
            let group: Group<K, V> = unsafe { self.groups.add(offset).into() };
            for entry_index in meta.match_tag(&h2) {
                if group[entry_index].0 == *key {
                    return unsafe {
                        let ptr = self.groups.add(offset + entry_index);
                        Some(&ptr.as_ref().1)
                    };
                }
            }

            if meta.match_empty().any_bit_set() {
                return None;
            }
            index = (index + 1) & self.mask; // we simply do linear probing
            if index == start_index {
                // all group probed
                return None;
            }
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V>
    where
        K: Copy,
    {
        if self.len == 0 {
            return None;
        }
        // TODO: use ProbSeq instead raw linear probing
        let (h1, h2) = self.hash(key);
        let start_index = h1 & self.mask;
        let mut index = start_index;
        loop {
            let offset = index as usize * GROUP_SIZE;
            // First: try find a entry for key
            let meta: Metadata = unsafe { self.metas.add(offset).into() };
            let group: Group<K, V> = unsafe { self.groups.add(offset).into() };
            for entry_index in meta.match_tag(&h2) {
                if group[entry_index].0 == *key {
                    self.len -= 1;
                    let (_, value) = unsafe {
                        ptr::write(self.metas.add(offset + entry_index).as_ptr(), Tag::DELETED);
                        ptr::read(self.groups.add(offset + entry_index).as_ptr())
                    };
                    return Some(value);
                }
            }

            if meta.match_empty().any_bit_set() {
                return None;
            }
            index = (index + 1) & self.mask; // we simply do linear probing
            if index == start_index {
                // all group probed
                return None;
            }
        }
    }
}

impl<K, V> SwissTable<K, V> {
    fn drain(&mut self) -> Drain<(K, V)> {
        let len = self.len;
        self.len = 0;
        Drain {
            index: 0,
            len,
            meta: self.metas,
            group: self.groups,
            bitmask_iter: Metadata::from(self.metas).match_full().into_iter(),
        }
    }
}

impl<K, V> Drop for SwissTable<K, V> {
    fn drop(&mut self) {
        for _ in self.drain() {}
        let metas_layout = Layout::array::<u8>(NUMBER_GROUP * GROUP_SIZE).unwrap();
        let groups_layout = Layout::array::<(K, V)>(NUMBER_GROUP * GROUP_SIZE).unwrap();
        unsafe {
            alloc::dealloc(self.metas.as_ptr() as *mut _, metas_layout);
            alloc::dealloc(self.groups.as_ptr() as *mut _, groups_layout);
        }
    }
}

struct Drain<T> {
    index: usize,
    len: usize,
    meta: NonNull<u8>,
    group: NonNull<T>,
    bitmask_iter: BitMaskIter,
}

impl<T> Iterator for Drain<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else if let Some(slot_index) = self.bitmask_iter.next() {
            self.len -= 1;

            //* Safe: we are iterating metadata linearly, and the table will be empty eventually
            unsafe { ptr::write(self.meta.add(slot_index).as_ptr(), Tag::EMPTY) };

            Some(unsafe { ptr::read(self.group.add(slot_index).as_ptr()) })
        } else if self.index < NUMBER_GROUP - 1 {
            self.index += 1;
            unsafe {
                self.meta = self.meta.add(GROUP_SIZE);
                self.group = self.group.add(GROUP_SIZE);
                self.bitmask_iter = Metadata::from(self.meta).match_full().into_iter();
            }
            self.next()
        } else {
            None
        }
    }
}

impl<T> Drop for Drain<T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

#[derive(Default)]
struct DefaultHashBuilder {}

impl BuildHasher for DefaultHashBuilder {
    type Hasher = SipHasher13;
    fn build_hasher(&self) -> Self::Hasher {
        // TODO: random state
        SipHasher13::new()
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

/// Metadata is a 16-byte for a group a entry.
struct Metadata {
    m128i: x86::__m128i,
}

impl From<NonNull<u8>> for Metadata {
    fn from(value: NonNull<u8>) -> Self {
        Self {
            m128i: unsafe { x86::_mm_load_si128(value.as_ptr() as *const _) },
        }
    }
}

impl Metadata {
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

/// Tag is a metadata control byte for a slot in group.
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
#[cfg(target_arch = "x86_64")]
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
            let meta = _mm_load_si128(ptr as *const _);
            // SIMD instruction, match EMPTY or DELETED byte
            // since EMPTY or DELETE byte set highest bit to 1
            // use SIMD to collect bytes with highest bit set to 1
            let mask = _mm_movemask_epi8(meta) as u16;
            // bits with 1 are empty or delete
            assert_eq!(mask, 0b0011_1010_1010_1111);
        }
    }

    #[test]
    fn test_hash_brown_map() {
        // use hashbrown for debugging, since std lib jump into assembly code at some points
        let mut m = hashbrown::HashMap::with_capacity(16);

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
