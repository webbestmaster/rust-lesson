fn main() {
    test();
    println!("End of main")
}

fn test() {
    let a = MyStruct {
        v: 1,
        s: Box::new(
            Some(MyStruct {
                v: 2,
                s: Box::new(None),
            })
        ),
    };
    println!("End of test")
}

struct MyStruct {
    v: i32,
    s: Box<Option<MyStruct>>,
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Cleaning {}", self.v)
    }
}
