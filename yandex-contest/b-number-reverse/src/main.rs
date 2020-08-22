use std::io;

fn main() {
    let mut number_str = String::new();

    io::stdin().read_line(&mut number_str).expect("Wring file, should contain 1 line with numbers");

    println!("number str {}", number_str.trim());

    let mut char_list = number_str.trim().chars();

    let first_char = char_list.nth(0).expect("Wrong number, should start with number of minus");

    let is_negative = first_char == '-';

    let mut result = if is_negative {
        vec![first_char]
    } else {
        Vec::new()
    };

    loop {
        let current_char = match char_list.next_back() {
            Some(char) => char,
            None => break
        };

        if current_char != '0' {
            result.push(current_char);
        }
    }

    if !is_negative {
        result.push(first_char);
    }

    let result_str: String = result.into_iter().collect();

    println!("result str {:?}", result_str);
}
