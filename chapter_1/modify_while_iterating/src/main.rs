fn main() {
    let mut letters = vec![
        "a", "b", "b"
    ];

    for letter in letters {
        println!("{}", letter);
        // fails compilation, because rust will not allow letters
        // to be modified within iteration block
        letters.push(letter.clone())
    }
}