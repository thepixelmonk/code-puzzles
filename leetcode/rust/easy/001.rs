use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        
        for i in 0..nums.len() {
            let remainder = (target - nums[i]);
            
            if let Some(idx) = map.get(&remainder) {
                return vec![i as i32, *idx as i32];
            } else {
                map.insert(nums[i], i);
            }
        }
        
        Vec::new()
    }
}
