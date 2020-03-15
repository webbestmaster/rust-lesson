fn to_vec(a: i32, b: i32) -> Vec<i32> {
    vec![a, b]
}

fn main() {
    let vec = to_vec(2, 3);

    println!("vec {:?}", vec);
}
