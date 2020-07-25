mod spores;
mod plant;

fn main() {
    println!("Hello, world!");

    let spore: spores::Spore = spores::produce_spore("spore");

    println!("{:?}", spore);

    let plant: plant::Plant = plant::produce_plant("plant");

    println!("{:?}", plant);

    let leave_item = plant::leave::Leave {};

    println!("{:?}", leave_item);

}
