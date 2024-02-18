fn main() {
    // ';' are required or the code won't compile.

    // Create a 16bit integer.
    let x: i16 = 10;

    // error | 10.0 is a float not a 16bit integer
    // let x: i16 = 10.0;
    
    // Variables a immutable by default.
    
    // error | cannot change value without 'mut'
    // x = 11;

    // Mutable Variables - allow a variable to change its value (not its type).
    let mut y: i16 = 11;
    
    // Rust code compiles if values are not used but a warning appears:
    // warning: unused variable: `x`
    // warning: variable `y` is assigned to, but never used

    println!("y is: {y}");

    y = 12;
    println!("y is now: {y}");

    // Print - print value to stdout.
    println!("x is: {x}");

    // Shadowing - two or more distinct declarations of a variable with the same name.
    let a: i32 = 10;
    let a: i32 = a+3;
    println!("a is: {a}");

    // Scope - isolate variables within it's own scope.
    {
        let a: i32 = a+10;
        println!("a is: {a}");
    }
    println!("a is: {a}");

    // Constants - a convenient alternative to code duplication.
    // Sometimes a certain value is used many times throughout a program, 
    // and it can become inconvenient to copy it over and over. What's more,
    // it's not always possible or desirable to make it a variable that gets
    // carried around to each function that needs it. 
    const MAX_VALUE: u32 = 100;
    println!("MAX_VALUE is: {MAX_VALUE}");
}
