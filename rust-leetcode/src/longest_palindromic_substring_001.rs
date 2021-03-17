/*
https://leetcode-cn.com/problems/longest-palindromic-substring/
给定一个字符串 s，找到 s 中最长的回文子串。你可以假设 s 的最大长度为 1000。

示例 1：

输入: "babad"
输出: "bab"
注意: "aba" 也是一个有效答案。
示例 2：

输入: "cbbd"
输出: "bb"

*/
/*
思路:
怎么把规模大的问题化成规模小的问题进行解决
假设用m[i][j]表示从i到j是回文的长度
那么只有两种情况可以扩展出回文
m[i][j]是回文,当且仅当:
1. m[i][j-1]是回文,并且m[i][j-1]长度是1,并且m[j-1]==m[j]  例如：输入: "cbbd"  输出: "bb"
2. m[i+1][j-1]是回文,并且m[i]==m[j]                      例如：输入: "babad"  输出: "bab"
遍历的过程中记一个最长字符串即可.
*/

struct Solution {}

impl Solution {
    // 动态规划
    // 穷举法的时间复杂度过高，接下来我们用DP进行优化。对于母串s，我们用c[i,j]=1表示子串s[i..j]为回文子串，那么就有递推式
    //
    // c[i,j]={
    //      c[i+1,j−1]; if s[i]=s[j]
    //      0;          if s[i]≠s[j]
    // }
    // 上述递推式的含义：当s[i]=s[j]时，如果s[i+1..j-1]是回文子串，则s[i..j]也是回文子串；
    // 如果s[i+1..j-1]不是回文子串（或s[i]≠s[j]），则s[i..j]也不是。
    // 特别地，对于这样的字符串——只包含单个字符、或者两个字符重复，其均为回文串：
    // c[i,i] = 1
    // c[i,i+1] = 1 if s[i] == s[i+1]

    pub fn longest_substr_001(input_str: String) -> String {
        if input_str.len() == 0 {
            return input_str;
        }
        let s = input_str.as_bytes();
        let mut c = vec![vec![0; s.len()]; s.len()];

        // 单个字符
        for i in 0..c.len() {
            c[i][i] = 1;
        }

        let mut longest = &s[0..1];
        // step
        for gap in 1..input_str.len() {
            // 元素下标
            for i in 0..input_str.len() {
                let j = i + gap;
                // 两个重复
                if j < input_str.len() && c[i][j - 1] == 1 && s[j - 1] == s[j] {
                    c[i][j] = 2;
                    if longest.len() <= gap {
                        longest = &s[i..(j + 1)]; // 包含最后一位
                    }
                    continue;
                }

                if i + 1 >= c.len() || j >= c.len() {
                    continue; // 越界的不考虑
                }
                if c[i + 1][j - 1] > 0 && s[i] == s[j] {
                    c[i][j] = c[i + 1][j - 1] + 2; // 向周边扩展了两位
                    if longest.len() <= gap {
                        longest = &s[i..(j + 1)]; // 包含最后一位
                    }
                }
            }
        }
        String::from_utf8(Vec::from(longest)).ok().unwrap()
    }

    // 回文串是左右对称的，如果从中心轴开始遍历，会减少一层循环。
    // 思路：依次以母串的每一个字符为中心轴，得到回文串；然后通过比较得到最长的那一个。
    // 注意：要考虑到像baccab这样的特殊子串，可以理解它的中心轴是cc。

    pub fn l2r_helper(input_str: &str, mid: usize) -> String {
        let mut r = mid + 1;
        let len = input_str.len();
        let s = input_str.as_bytes();

        while r < len && s[r] == s[mid] {
            r += 1;
        }
        if mid == 0 {
            let result = &s[mid..r];
            return String::from_utf8(Vec::from(result)).ok().unwrap();
        }

        let mut l = mid - 1;
        while l >= 0 && r < len && s[l] == s[r] {
            if l <= 0 {
                let result = &s[l..r + 1];
                return String::from_utf8(Vec::from(result)).ok().unwrap();
            }
            l -= 1;
            r += 1;
        }

        let result = &s[l + 1..r];
        String::from_utf8(Vec::from(result)).ok().unwrap()
    }

    pub fn longest_substr_002(input_str: String) -> String {
        let len = input_str.len();
        let s = input_str.as_bytes();
        let tmp = input_str.as_str();
        let mut logest = String::from("");
        for i in 0..len {
            let l = Self::l2r_helper(tmp, i);
            println!(".........{:?}", l);
            if l.len() > logest.len() {
                logest = l;
            }
        }
        logest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // let input_str = String::from("abcdcba");
        // let sub_str = Solution::longest_substr_002(input_str);
        // println!("====sub string: {:?}", sub_str);

        // let input_str = String::from("abcdcba");
        // let sub_str = Solution::longest_palindrome(input_str);
        // println!("====sub string1: {:?} {:?}",input_str ,sub_str);
        //
        // let input_str = String::from("babad");
        // let sub_str = Solution::longest_palindrome(input_str);
        // println!("====sub string2: {:?} {:?}",input_str ,sub_str);
        //
        // let input_str = String::from("bb");
        // let sub_str = Solution::longest_palindrome(input_str);
        // println!("====sub string3: {:?} {:?}",input_str ,sub_str);

        // let mut input_str = String::from("baccab");
        // println!("====sub string input: {:?} ", "baccab");
        // let sub_str = Solution::longest_palindrome(input_str);
        // println!("====sub string4: {:?} {:?}", "baccab", sub_str);

        let mut input_str = String::from("eeabba");
        println!("====sub string input: {:?} ", "baccab");
        let sub_str = Solution::longest_substr_001(input_str);
        println!("====sub string4: {:?} {:?}", "baccab", sub_str);
    }
}
