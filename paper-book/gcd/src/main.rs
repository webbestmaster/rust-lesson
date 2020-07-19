use std::io::Write;
use std::str::FromStr;
use std::env;

// TODO: research expect vs unwrap

fn main() {
    let mut numbers = Vec::new();

    println!("-----------");

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing arguments"));
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];

    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("{}", gcd(234, 567));
    println!("Greatest common divisor of {:?} is {}", numbers, d);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            m = m + n;
            n = m - n;
            m = m - n;
        }
        m = m % n;
    }

    return n;
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
