// https://leetcode.com/problems/two-sum/
fn main() {
    let result = two_sum(vec![1, 2, 3], 5);
    println!("{:?}", result);
    assert_eq!(result, vec![1, 2]);

    println!("Success!");
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (index_a, first) in nums.iter().enumerate() {
        for (index_b, second) in nums.iter().enumerate() {
            if first + second == target && index_a != index_b {
                return vec![index_a as i32, index_b as i32];
            }
        }
    }

    return vec![];
}
