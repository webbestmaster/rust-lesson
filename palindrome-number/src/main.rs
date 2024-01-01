// https://leetcode.com/problems/palindrome-number/
struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x_as_string: String = x.to_string();
        let reversed_x: String = x_as_string.chars().rev().collect::<String>();

        return x_as_string == reversed_x;
    }
}

fn main() {
    let is_pal: bool = Solution::is_palindrome(121);

    println!("{is_pal}");
}

