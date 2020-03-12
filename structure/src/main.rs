struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = build_user(String::from("me@mail.com"), String::from("mike"));

    let first = first_word(&user1.email);

    println!("first: {}", first);

    let rectangle = Rectangle { width: 20, height: 40 };

    println!("area: {}", rectangle.area(20));

    let square = Rectangle::square(50);
    println!("area of square = {}", square.area(2));
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self, multi: u32) -> u32 {
        self.height * self.width * multi
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { height: size, width: size }
    }
}
