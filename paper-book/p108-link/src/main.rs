static mut STASH: &i32 = &1;

fn main() {
    println!("Hello, world!");

    let list = vec![6, 5, 4, 1, 2, 3];
    let small = smallest(&list);
    println!("small {}", small);

    mut_no_mut();
}

// the same
// fn smallest<'a>(v: &'a [i32]) -> &'a i32 {
fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s { s = r; }
    }
    s
}

fn mut_no_mut() {
    let mut x = 42;
    let p = &x;
    assert_eq!(*p, 42);

    // x += 1;
    // assert_eq!(*p, 42);
}
