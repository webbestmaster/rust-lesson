// enum Option, Some and None got from std
//enum Option<T> {
//    Some(T),
//    None,
//}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let some_number = Some(5);
    let mut some_string = Some("a string");
    let absent_number: Option<i32> = None;

    some_string = None;

    let result_absent_number: i32 = match absent_number {
        Option::Some(value) => value,
        Option::None => 0,
    };

    let result_some_string: &str = match some_string {
        Option::Some(value) => value,
        Option::None => "",
    };

    println!("absent_number: {}", result_absent_number);
    println!("result_some_string: {}", result_some_string);

    let my_coin: Coin = Coin::Nickel;

    println!("my coin {}", value_in_cents(my_coin));
}


fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
