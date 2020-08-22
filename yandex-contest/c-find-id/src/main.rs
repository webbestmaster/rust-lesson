use std::io;

fn main() {
    let mut list_length_str = String::new();

    io::stdin().read_line(&mut list_length_str).expect("Wrong file, should contain 2 line with numbers");

    let list_length = list_length_str.trim().parse::<usize>().expect("Wrong file, should contain 2 line with numbers");

    let mut number_line_str = String::new();

    io::stdin().read_line(&mut number_line_str).expect("Wrong file, should contain 2 line with numbers");

    let number_line_str_padded = [' '.to_string(), number_line_str.trim().to_string(), ' '.to_string()].concat();

    for number in 1..list_length + 1 {
        let number_mask = [' '.to_string(), number.to_string(), ' '.to_string()].concat();

        if !number_line_str_padded.contains(&number_mask) {
            print!("{} ", number);
        }
    }

    println!("");
}
