fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);
}

// Compiling veriables v0.1.0 (D:\Works\com.14bytes\909Plan\RustyMaterials\veriables)
// error[E0384]: cannot assign twice to immutable variable `x`
// --> src\main.rs:5:5
// |
// 2 |     let x = 5;
// |         -
// |         |
// |         first assignment to `x`
// |         help: consider making this binding mutable: `mut x`
// ...
// 5 |     x = 6;
// |     ^^^^^ cannot assign twice to immutable variable
//
// For more information about this error, try `rustc --explain E0384`.
// error: could not compile `veriables` (bin "veriables") due to previous error
