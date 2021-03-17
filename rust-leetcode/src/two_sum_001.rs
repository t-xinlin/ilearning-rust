/*
1. 两数之和
给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。

你可以假设每种输入只会对应一个答案。但是，你不能重复利用这个数组中同样的元素。

示例:

给定 nums = [2, 7, 11, 15], target = 9

因为 nums[0] + nums[1] = 2 + 7 = 9
所以返回 [0, 1]

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/two-sum
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
一遍遍历,建立一个值=>下标的map
同时查找历史中是否有满足的,有的话输出
题中说了只有一对
*/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(inputs: Vec<i32>, target: &i32) -> Vec<i32> {
        let mut m = HashMap::new();

        for input in inputs.iter().enumerate() {
            let t1 = target - input.1;
            if let Some(t2) = m.get(&t1) {
                return vec![*t2 as i32, input.0 as i32];
            }
            m.insert(input.1, input.0);
        }
        Vec::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let tar = 9;
        assert_eq!(Solution::two_sum(vec![2, 11, 7, 15], &tar), vec![0, 1]);

        let tar = 9;
        assert_eq!(Solution::two_sum(vec![2, 11, 7, 15], &tar), vec![0, 2]);
    }
}
