pub mod markdown;

use markdown::Config;

fn main() {
    let config = Config {
        index: 1
    };

    println!("{:?}", config);

    println!("Hello, world!");
}
