fn main() {
    let twenty = 20; // inferred type
    let twenty_one: i32 = 21; // type annotations
    let twenty_two = 22i32; // type suffixes

    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    // underscores increase readability and are ignored by compiler
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2)); // method call raises to power

    let forty_twos = [ // arrays must contain same type
        42.0,
        42f32,
        42.0_f32,
    ];

    println!("{:02}", forty_twos[0]);
}