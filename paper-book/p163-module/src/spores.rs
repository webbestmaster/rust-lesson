#[derive(Debug)]
pub struct Spore {

}

pub fn produce_spore(factory: &str) -> Spore {
    println!("name {}", factory);

    Spore {}
}
