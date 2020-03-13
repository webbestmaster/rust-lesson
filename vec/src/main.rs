#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v: Vec<i32> = Vec::new();
    print!("{:?} ", v);

    let v: Vec<i32> = vec![2];
    println!("{:?}", v);

    let v = vec![1, 2, (3 as u64)];
    println!("{:?}", v);

    //let mut v = Vec::new();
    let mut v = vec![];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.pop();

    println!("{:?}", v);

    let third: i32 = v[2];
    println!("{}", third);

    let third: Option<&i32> = v.get(2);
    println!("{:?}", third);

    let mut v = vec![1, 2, 3, 4, 5];
    let first = v[0];
    v.push(6);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);
}
