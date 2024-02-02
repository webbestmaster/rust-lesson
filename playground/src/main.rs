struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut result: String = String::new();

        let vowels = "aeiouAEIOU";

        let mut vowels_from_arg = String::new();

        for c in s.chars() {
            if vowels.contains(c) {
                vowels_from_arg.push(c);
            }
        }

        let mut vowels_from_arg_index = vowels_from_arg.len() - 1;

        let vowel_chars = vowels_from_arg.chars().collect::<Vec<char>>();
        let mut char_from_arg: &char;

        for c in s.chars() {
            if vowels.contains(c) {
                char_from_arg = vowel_chars.get(vowels_from_arg_index).unwrap();
                vowels_from_arg_index -= 1;
                result.push(*char_from_arg);
            } else {
                result.push(c);
            }
        }

        return result;
    }
}

fn main() {
    println!("{:?}", Solution::reverse_vowels("Hello".to_owned()));
}
