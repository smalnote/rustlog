mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn permute(nums: &[i32]) -> String {
    utils::set_panic_hook();
    let nums = nums.into();
    let result = permute::permute(nums);
    serde_json::to_string(&result).unwrap()
}

mod permute {
    use std::collections::hash_set::HashSet;
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut track = Vec::with_capacity(nums.len());
        let mut used = Default::default();
        let mut result = Default::default();

        backtrack_permute(&nums, &mut used, &mut track, &mut result);
        result
    }

    fn backtrack_permute(
        nums: &Vec<i32>,
        used: &mut HashSet<i32>,
        track: &mut Vec<i32>,
        results: &mut Vec<Vec<i32>>,
    ) {
        if track.len() == nums.len() {
            let result = track.clone();
            results.push(result);
            return;
        }

        for num in nums {
            if !used.contains(num) {
                used.insert(*num);
                track.push(*num);
                backtrack_permute(nums, used, track, results);
                track.pop();
                used.remove(num);
            }
        }
    }
}
