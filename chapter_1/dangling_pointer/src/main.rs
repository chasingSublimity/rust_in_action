#[derive(Debug)]
enum Cereal {
    Barley, Millet, Rice, 
    Rye, Spelt, Wheat,
}

fn main() {
    // init an empty vector with elements of type Cereal
    let mut grains: Vec<Cereal> = vec![];
    grains.push(Cereal::Rye);
    drop(grains);

    // main.rs does not compile because of this line.
    println!("{:?}", grains)
}
