// https://leetcode.com/problems/valid-parentheses/

use std::collections::HashMap;
use std::str::Chars;
use std::string::ToString;

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut parentheses = String::new();

        let mut parentheses_map: HashMap<char, char> = HashMap::from([
            (')', '('),
            ('}', '{'),
            (']', '['),
        ]);

        let char_list: Chars = s.chars();
        let mut last_open: char;

        for inner_char in char_list {
            for open in "{[(".chars() {
                if open == inner_char {
                    parentheses.push(open);
                    last_open = open;
                }
            }

            for close in "}])".chars() {
                if close == inner_char {
                    let open_char = parentheses_map.get(&close).unwrap_or(&'1');
                    let last_open_char = parentheses.chars().last().unwrap_or('2');

                    if *open_char == last_open_char {
                        parentheses.pop();
                        continue;
                    }

                    return false;
                }
            }
        }

        return parentheses.is_empty();
    }
}

fn main() {
    let is_valid: bool = Solution::is_valid("([)]".to_string());

    println!("{is_valid}");
}

