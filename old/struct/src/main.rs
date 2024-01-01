struct User {
    email: String,
    username: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.length * self.width;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[derive(Debug)]
enum TrafficLight {
    Red1,
    Yellow,
    Green,
}

use TrafficLight::{Red1, Yellow};

fn main() {
    let mut user1 = build_user(String::from("someone@example.com"), String::from("someusername123"));
    user1.email = String::from("anotheremail@example.com");
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    println!("[{};{};{};{}]", user1.username, user1.email, user1.active, user1.sign_in_count);
    println!("[{};{};{};{}]", user2.username, user2.email, user2.active, user2.sign_in_count);

    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };
    println!("area of rect1 = {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let red = Red1;
    let yellow = Yellow;
    let green = TrafficLight::Green;

    println!("{:?}", red);
    println!("{:?}", yellow);
    println!("{:?}", green);
}
