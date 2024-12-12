// 方法1:
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..(nums.len() - 1) {
            for j in (i + 1)..(nums.len()) {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32]
                }
            }
        }
        vec![]
    }
}

// 方法2:
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        match map.get(&(target - num)) {
            Some(&j) => return vec![j,i as i32],
            None => map.insert(num, i as i32),
        };
    }
    return vec![]
    }
}
