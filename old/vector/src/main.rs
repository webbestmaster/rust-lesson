#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);

    let first = i32::from(2);

//    v.push(6);

    println!("{:?}", v);
    println!("{}", first);

//    let mut v2: Vec<Rectangle> = Vec::new();
    let mut v2: Vec<Rectangle> = Vec::new();
    v2.push(Rectangle { length: 22, width: 33 });

    let v2first = &v2[0];
//    let v2sec = v2[0];

    println!("{}", v2first.length);
//    println!("{}", v2sec.length);
}
