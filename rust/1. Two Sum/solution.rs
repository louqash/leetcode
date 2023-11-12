use std::convert::TryInto;
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut store: HashMap<i32, usize> = HashMap::new();
        for index in 0..nums.len() {
            let diff: i32 = target - nums[index];
            if let Some(&other_index) = store.get(&nums[index]) {
                return vec![
                    index.try_into().unwrap(),
                    other_index.try_into().unwrap()
                ];
            } else {
                store.insert(diff, index);
            }
        }
        vec![]
    }
}

