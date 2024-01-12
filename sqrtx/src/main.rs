struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x > i32::MAX {
            return Self::my_sqrt(i32::MAX);
        }

        let mut result: i32 = 0;
        let mut sugges: i32 = 0;

        for i in 0..=x {
            sugges = i * i;
            if sugges == x {
                return i;
            }

            if sugges > x {
                return result;
            }

            result = i;
        }

        return result;
    }
}

fn main() {
    println!("{}", i32::MAX);
    println!("{}", i32::MAX);
    println!("{}", Solution::my_sqrt(2147395600));
}
