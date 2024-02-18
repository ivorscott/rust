fn main() {
    // Compound Data types

    // &str and String
    // &str is a sting slice 'immumtable', of fix size (useful for read only access)
    let fixed_str: &str = "Fixed length string";

    // The println! macro will lock the standard output on each call.
    println!("{fixed_str}");
    // String with a capital S 
    let mut flexible_str: String = String::from("This string will grow");
    // You can add a character to the string with push()
    flexible_str.push('!');

    // Arrays
    // All elements need to be of the same type
    // You can remove the type annotation and the rust compiler will be able to deduce the type itself.
    let mut array_1 = [4,5,6,7,8];
    let mut array_1: [i32;5] = [4,5,6,7,8];
    let num = array_1[3];

    // "{:?}" is a format specifier to print compound data types.
    println!("{:?}", array_1);
    // add default values 0...10
    let array_2 = [0;10];

    // Vectors
    // Just like arrays, all elements need to be of the same type.
    // But vectors can shrink or grow in size.
    let vec_1: Vec<i32> = vec![4,5,6,7,8];
    let num = vec_1[3];

    // Tuples
    // Tuples can hold values of different types
    let info = ("String", 3000, 3.0);

    // Empty Tuple
    let unit = ();
}
