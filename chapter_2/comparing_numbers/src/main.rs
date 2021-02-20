fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    // cast b in order to make comparison possible
    if a < (b as i32) { // safer to cast small number as larger one. also known as promotion
        println!("Ten is less than one hundred.")
    }
}
