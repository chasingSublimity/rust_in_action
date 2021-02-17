use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    // integer literal, stored on the stack
    let a = 10;

    // a boxed integer, stored on the heap
    let b = Box::new(20);

    // boxed integer wrapped in a reference counter
    let c = Rc::new(Box::new(30));

    // Integer protected by a mutex, wrapped in an atomic reference counter
    let d = Arc::new(Mutex::new(40));

    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d);
}
