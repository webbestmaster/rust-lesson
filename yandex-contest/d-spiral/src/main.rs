use std::io;

fn main() {
    let mut list_length_str = String::new();

    io::stdin().read_line(&mut list_length_str).expect("Wrong file, should contain several line with numbers");

    let list_length = list_length_str.trim().parse::<i32>().expect("Wrong file, should contain several line with numbers");

    println!("list_length {}", list_length);
}
