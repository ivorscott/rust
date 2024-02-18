fn main() {
    // if else statements
    let num = 10;

    let marks = 95;

    // single quotes for characters
    let mut grade = 'N';

    // if marks >= 90 {
    //     grade = 'A';
    // } else if marks >= 80 {
    //     grade = 'B';
    // } else if marks >= 70 {
    //     grade = 'C';
    // } else {
    //     grade = 'F'
    // }

    // let grade = if marks >= 90 {
    //     'A'
    // } else if marks >= 80 {
    //     'B'
    // } else if marks >= 70 {
    //     'C'
    // } else {
    //     'F'
    // };

    // .. represents a range of values
    // be careful of unreachable code paths
    // match marks {
    //     90..=100 => grade = 'A',  // 1st arm
    //     80..=89 => grade = 'B',  // 2nd arm
    //     70..=79 => grade = 'C',  // 3rd arm
    //     60..=69 => grade = 'D',  // 4th arm
    //     _ => grade = 'F', // default (match everything)
    // }

    let grade = match marks {
        90..=100 => 'A',  // 1st arm
        80..=89 => 'B',  // 2nd arm
        70..=79 => 'C',  // 3rd arm
        60..=69 => 'D',  // 4th arm
        _ => 'F', // default (match everything)
    };
}
