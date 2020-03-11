fn main() {
    let mut x: &str = "the x";

    plus_one(&mut x);
    println!("The value of x is: {}", x);

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    let mut string = String::from("Hello");
    add_world(&mut string);
    println!("The value of x is: {}", string);

    let a: f32 = 0.1;
    let b: f32 = 0.2;
    println!("f32: 0.1 + 0.2 = {}", a + b);

    let a: f64 = 0.1;
    let b: f64 = 0.2;
    println!("f64: 0.1 + 0.2 = {}", a + b);

    let s1 = String::from("hello");
    let s2 = s1;

    let mut s = String::from("hello");
    let r1 = &mut s;
    println!("{}", r1);

    let r2 = &mut s;
    println!("{}", r2);

//    println!("The value of x is: {}", s1); // will not works!!!
    println!("The value of x is: {}", s2);
}

fn plus_one(x: &str) -> i32 {
    println!("The value of x is: {}", x);

    return 1;
}

fn add_world(string: &mut String) {
    string.push_str(", world!");
}
