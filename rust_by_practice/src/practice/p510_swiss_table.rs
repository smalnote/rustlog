// x86_64 SIMD as example
#[cfg(test)]
#[cfg(target_arch = "x86_64")]
mod tests {
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
                EMPTY, DELETED, EMPTY, EMPTY, EMPTY,
            ];
            // since we only use tags ptr later, visit it make sure it is initialized
            for _ in tags {}

            // get slice start pointer, not fat array pointer
            let ptr = &tags[..];
            let ptr = ptr.as_ptr();
            // construct a control of 16 bytes from raw bytes
            let ctrl = _mm_load_si128(ptr as *const _);
            // SIMD instruction, match EMPTY or DELETED byte
            // since EMPTY or DELETE byte set highest bit to 1
            // use SIMD to collect bytes with highest bit set to 1
            let mask = _mm_movemask_epi8(ctrl) as u16;
            // bits with 1 are empty or delete
            assert_eq!(mask, 0b1111_1010_1010_1111);
        }
    }

    #[test]
    fn test_hash_brown_map() {
        // use hashbrown for debugging, since std lib jump into assembly code at some points
        let mut m = hashbrown::HashSet::with_capacity(16);

        m.insert(10);
        m.insert(20);
        m.insert(30);

        m.remove(&20);
        m.remove(&10);

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
