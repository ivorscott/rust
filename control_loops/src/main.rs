fn main() {
    // loop forever

    // loop {
    //     println!("Simple loop");
    // }

    // break out of loop with 'break'

    // loop {
    //     println!("Simple loop");
    //     break
    // }

    // if there are nested loops 'break' only
    // breaks out of the inner loop

    // use leveling to instruct the compiler to explicitly
    // break out of the outer loop

    // 'name:   starts the level
    // break 'name; ends 

    // basically you give a loop a name and explicitly 
    // reference it when using break

    'outer: loop {
        println!("Simple loop");
        break 'outer;
    }

    // you can return a value on break
    let a = loop {
        break 5;
    };

    // for loop
    let vec = vec![45,50,35,23,46];
    for i in vec {
        println!("{i}");
    }

    // while loop
    let mut num = 0;
    while num < 10 {
        num = num +1;
    }
}
