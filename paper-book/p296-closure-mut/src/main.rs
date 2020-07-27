fn call_closure_mut<F>(mut closure: F)
    where F: FnMut() // FnMut - make mutable closure
{
    closure()
}

fn main() {
    let mut i = 0;

    call_closure_mut(|| i += 1);

    println!("i :{}", i);
}



