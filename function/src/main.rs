fn main() {
    let mut x = String::from("the x");


    plus_one(& mut x);
    println!("The value of x is: {}", x);
}

fn plus_one(x: & mut str) -> i32 {

    println!("The value of x is: {}", x);

    return 1;
}
