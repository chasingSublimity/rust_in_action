use std::thread;

fn main() {
    let mut data = 100;

    // these two lines cause an error
    thread::spawn(|| { data = 500; });
    thread::spawn(|| { data = 1000; });

    // as does this line
    println!("{}", data);
}
