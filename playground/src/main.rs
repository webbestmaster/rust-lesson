#[derive(Debug)]
struct SomeStruct {
    val: i32,
}

fn main() {
    let mut arr:Vec<SomeStruct> = Vec::new();



    arr.push(SomeStruct {val: 1});

    let mm: &mut SomeStruct = arr.last_mut().unwrap();

    mm.val = 2;

    let mm1: &SomeStruct = arr.last().unwrap();


    // let ff = arr[1];

    // println!("{:?}", mm);
    println!("{:?}", arr);
}
