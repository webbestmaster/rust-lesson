use std::io;

fn main() {
    let mut list_length_str = String::new();

    io::stdin().read_line(&mut list_length_str).expect("Wring file, should contain 3 line with numbers");

    let list_length = match list_length_str.trim().parse::<i32>() {
        Ok(number) => number,
        Err(error) => {
            println!("This was not an integer: {}", list_length_str);
            println!("Error: {:?}", error);
            return;
        }
    };

    println!("List length (not needed) {}", list_length);

    let mut line_a = String::new();

    std::io::stdin().read_line(&mut line_a).expect("Wring file, should contain 3 line with numbers");

    let mut line_b = String::new();

    std::io::stdin().read_line(&mut line_b).expect("Wring file, should contain 3 line with numbers");

    let mut char_list_a = line_a.trim().chars();
    let mut char_list_b = line_b.trim().chars();

    let mut is_active_a = true;

    loop {
        let current_char_option = if is_active_a {
            char_list_a.next()
        } else {
            char_list_b.next()
        };

        match current_char_option {
            Some(char) => {
                print!("{}", char);

                if char == ' ' {
                    is_active_a = !is_active_a;
                }
            }
            None => {
                println!(" {}", char_list_b.as_str());
                break;
            },
        };
    }
}
