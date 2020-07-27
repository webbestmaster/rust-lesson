fn count(list: &[i32]) -> i32 {
    let mut counter: i32 = 0;

    for num in list {
        counter += 1 + num;
    }

    return counter;
}


fn main() {
    let int_list = vec!(1, 2, 3, 4, 5);

    let list_length = count(&int_list);

    println!("{}", list_length);
}
