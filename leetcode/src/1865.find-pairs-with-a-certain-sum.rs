use std::collections::HashMap;

pub struct FindSumPairs {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    nums2_count: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindSumPairs {
    pub fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut nums2_count = HashMap::new();
        nums2.iter().for_each(|&num| {
            nums2_count
                .entry(num)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });
        Self {
            nums1,
            nums2,
            nums2_count,
        }
    }

    pub fn add(&mut self, index: i32, val: i32) {
        let old = self.nums2[index as usize];
        let new = old + val;
        self.nums2_count.entry(old).and_modify(|count| *count -= 1);
        self.nums2_count
            .entry(new)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        self.nums2[index as usize] = new;
    }

    pub fn count(&self, tot: i32) -> i32 {
        let mut count = 0;
        for &num in self.nums1.iter() {
            count += *self.nums2_count.get(&(tot - num)).unwrap_or(&0);
        }
        count
    }
}
