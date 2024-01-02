// https://leetcode.com/problems/longest-common-prefix/

use std::string::ToString;

struct Solution;

impl Solution {
    const DEFAULT_WORD: &'static str = "";

    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut longest_prefix = "".to_string();
        let mut letter_index: usize = 0;

        if strs.len() == 0 {
            return longest_prefix;
        }

        let first_word: &str = match strs.first() {
            Some(word) => word,
            None => Self::DEFAULT_WORD,
        };

        loop {
            match first_word.chars().nth(letter_index) {
                None => return longest_prefix,
                Some(main_char_by_index) => {
                    for word in strs.iter() {
                        let char_by_index = word.chars().nth(letter_index).unwrap_or('_');
                        if char_by_index != main_char_by_index {
                            return longest_prefix;
                        }
                    }

                    longest_prefix.push(main_char_by_index);
                    letter_index += 1;
                }
            }
        }
    }
}

fn main() {
    let longest_prefix: String = Solution::longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]);

    println!("{longest_prefix}");
}

