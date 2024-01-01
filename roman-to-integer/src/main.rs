// https://leetcode.com/problems/roman-to-integer/

struct Solution;

impl Solution {
    pub fn get_value_by_char(letter: char) -> i32 {
        return match letter {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => {
                println!("can not read char: {letter}");
                return 0;
            }
        };
    }
    pub fn roman_to_int(s: String) -> i32 {
        let chars = s.chars();
        let mut sum: i32 = 0;
        let mut prev_number: i32 = 0;

        for i in chars {
            let current_value = Self::get_value_by_char(i);
            if prev_number < current_value {
                sum -= prev_number * 2;
            }
            sum += current_value;
            prev_number = current_value;
        }

        return sum;
    }
}

fn main() {
    let is_pal: i32 = Solution::roman_to_int(String::from("MCMXCIV"));

    println!("{is_pal}");
}

