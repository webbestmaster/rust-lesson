fn main() {
    println!("Hello, world!");
    println!("{}", gcd(234, 567));
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
