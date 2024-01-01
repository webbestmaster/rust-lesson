#[derive(Debug)]
pub struct Plant {}

pub fn produce_plant(factory: &str) -> Plant {
    println!("name {}", factory);

    Plant {}
}

pub mod leave;
