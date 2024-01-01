mod modules;

use modules::my;

fn main() {
    println!("Hello, world!");
    my::nested::function();
}
