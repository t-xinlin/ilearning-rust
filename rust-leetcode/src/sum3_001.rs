/*
给定一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a，b，c ，使得 a + b + c = 0 ？找出所有满足条件且不重复的三元组。

注意：答案中不可以包含重复的三元组。

例如, 给定数组 nums = [-1, 0, 1, 2, -1, -4]，

满足要求的三元组集合为：
[
  [-1, 0, 1],
  [-1, -1, 2]
]
*/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let mut answer = Vec::new();
        let mut flag = HashMap::new();
        for i in 1..nums.len() {
            flag.insert(nums[i], i);
        }
        let length = nums.len();
        for i in 1..length {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in i + 1..length {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }

                let x = 0 - nums[i] - nums[j];
                println!("=========flag:{:?}", flag);
                if let Some(y) = flag.get(&x) {
                    if *y > j {
                        answer.push(vec![nums[i], nums[j], x])
                    }
                }
            }
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let t1 = vec![-1, 0, 1, 2, -1, -4];
        let result1 = Solution::three_sum(t1);
        println!("===result: {:?}", result1)
    }
}
