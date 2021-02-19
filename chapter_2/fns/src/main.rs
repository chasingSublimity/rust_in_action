fn main() {
    let a = 10;
    let b: i32 = 20;
    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));
    println!("( a + b ) + ( c + d ) = {}", e);
}

// type declarations are required when defining functions
fn add(i: i32, j: i32) -> i32 {
    // functions return the result of the last expression.
    // expressions, in rust, do not have semicolons
    i + j
}