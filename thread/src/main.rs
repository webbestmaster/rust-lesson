use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    let v: Vec<i32> = vec![1, 2, 3];
    let link_v = v.to_owned();

    let handle = thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }


        println!("Here's a vector: {:?}", link_v);
    });


    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    for received in rx {
        println!("Got: {}", received);
    }

    // println!("{:?}", v);

//    drop(v); // oh no!

   handle.join().ok();
}
