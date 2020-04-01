// use std::env;
use std::fs;
use std::thread;

fn main() {

    let file_name = "./src/main.rs";

    let thread = thread::spawn(move || -> String {
        let contents = fs::read_to_string(file_name)
            .expect("Something went wrong reading the file");

        return contents;
    });

    let thread2 = thread::spawn(|| -> String {
        let contents = fs::read_to_string("./Cargo.toml")
            .expect("Something went wrong reading the file!");

        return contents;
    });


    let result_thread2 = thread2.join().unwrap();
    println!("{}", result_thread2);

    let result_thread = thread.join().unwrap();
    println!("{}", result_thread);

    println!("Hello, world!");
}
