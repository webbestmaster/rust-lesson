// https://leetcode.com/problems/remove-duplicates-from-sorted-array/description/

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();

        return nums.len() as i32;
    }
}

fn main() {
    let mut nums = vec![1, 1, 2];

    println!("count: {:?}", Solution::remove_duplicates(&mut nums));
    println!("vec: {:?}", nums);
}
