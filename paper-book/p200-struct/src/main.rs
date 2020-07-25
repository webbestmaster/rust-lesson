use std::cell::RefCell;

pub mod man;
type Man = man::Man;

fn main() {
    println!("Hello, world!");

    let man_item = Man {
        age: 1,
        about: RefCell::new("me".to_string()),
    };

    {
        let about = man_item.about.borrow();
        println!("{:?}", about);
    }

    let mut about_mut = man_item.about.borrow_mut();

    about_mut.push_str(" too");

    println!("{:?}", man_item);
}
