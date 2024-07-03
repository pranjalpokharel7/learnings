fn another_function() {
    println!("another function");
}

fn another_function_with_params(x: u32) {
    println!("another function called with arg: {}", x);
}

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");

    // shadow variable x; data type assigned must be the same
    x = 6;
    println!("The shadowed value of x is: {x}");

    // you'll need to re-assign if you need to change the underlying 
    // data type of the variable
    let x = "Hello!";
    println!("The newly assigned value of x is: {x}");

    let y = "23";
    let _z: u32 = y.parse().expect("Not a number string!");

    // Integer overflow wrapping - if integer exceeds the value 
    // outside range for a given size (ex, trying to store 256 in u8), 
    // the program wraps it around to the starting value if you use 
    // wrapping_* operations (there are also other possible "wrappers")
    let f: u8 = 255;
    let _c = f.wrapping_add(2);
    println!("_c: {}", _c);

    // Tuple can be used to group a number of values with a variety of 
    // types into one compound type - once declared, they can not grow 
    // or shrink in size. Access tuple elements using the dot operator
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup.0;
    println!("five hundred in number is : {}", five_hundred);

    // Array is a collection of multiple values, however every element must be of the 
    // same size. Arrays in Rust have a fixed length (see vectors if you want a 
    // resizable array-like container).
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    another_function();
    another_function_with_params(_z);
    println!("five: {}", function_returns_5());
}

// Function can be defined after the main() function in the source 
// code (unlike C/C++ in this case). Rust only cares that the function 
// is defined somewhere in a scope that can be seen by the caller.
fn function_returns_5() -> i32 {
    5 // notice the lack of semi-colon here since this is an expression
}