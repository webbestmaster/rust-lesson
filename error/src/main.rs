use std::io;
use std::io::Read;
use std::fs;
use std::fs::File;

fn main() {
    let file_content = read_username_from_file();
    println!("file_content {:?}", file_content);
    println!("file_content {:?}", read_file_better());
    println!("file_content {:?}", read_file_best());
    println!("file_content {:?}", fs::read_to_string("hello.txt"));
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("./hello.txt");

    println!("read_username_from_file {:?}", f);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_file_better() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_file_best() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
