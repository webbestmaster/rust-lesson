struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let str_len: usize = haystack.len();
        let sub_str_len: usize = needle.len();

        if sub_str_len > str_len {
            return -1;
        }

        let max_index: usize = str_len - sub_str_len;

        for sub_index in 0..=max_index {
            // let inner: usize = sub_index;
            let slice: &str = &haystack[sub_index..(sub_str_len + sub_index)];

            if slice == needle {
                return sub_index as i32;
            }
        }

        return -1;
    }
}

fn main() {
    println!("Index: {}", Solution::str_str(String::from("sadbutsad"), String::from("butsass")));
}
