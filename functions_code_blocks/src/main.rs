fn main() {
    let str = "testing";
    my_fn(str);

    // Code block (Scope).
    // like functions code blocks can also have a return value.
    // format! macro is used for string formatting in rust.
    // like functions the last statement without a semicolon 
    // will be the return value.
    // Code block differ from functions: there are no explicit parameters 
    // and they are not designed for reuse.
    let name = {
       let first = "Adam"; 
       let last = "Smith";
       format!("{first} {last}")
    };
}

// function with string slice param.
fn my_fn(s: &str) {
    println!("{s}")
}
// function with implicit return.
// last line is implicitly returned (notice there's no semicolon).
// -> X denoted return type of function.
fn mutiply(num1: i32, num2: i32) -> i32 {
    println!("Multiplying");
    num1 * num2
}

// return multiple files
fn add_subtract(num1: i32, num2: i32) -> (i32, i32) {
    println!("Multiplying");
    (num1 + num2, num1 - num2)
}

// using return keyword
// return is not needed when the returned value is the
// last expression in the function. In this case the ; is omitted.
fn add_subtract2(num1: i32, num2: i32) -> (i32, i32) {
    println!("Multiplying");
    return (num1 + num2, num1 - num2);
}
