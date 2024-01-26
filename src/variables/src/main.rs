use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is:{x}");
    x = 6; // if we didn't wrote mut it would give error variable can't assign
    println!("The value of x is:{x}");

    const TWO_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; //const variable , everytime we declate const we need to give it's type
    println!("Printing the const:{TWO_HOURS_IN_SECONDS}");

    //shadowing variable
    let y = 6;
    let y = y + 1;

    //Inner scope
    {
        let y = y * 2;
        println!("The value of y in inner scope is:{y}");
    }
    println!("The value of y is:{y}");

    let spaces = "          ";
    let spaces = spaces.len();
    println!("Amount of length user asked for is:{spaces}");

    //Floting type value
    let p = 2.0;
    let q: f32 = 3.0;
    println!("Printing the above floting point numbers:{:.1},{:.1}", p, q);

    //Numeric Operations
    let addition = 5 + 10;
    let substraction = 93.2 - 41.9;
    let multiplication = 8 * 9;
    //division
    let quotient = 56.2 / 12.8;
    let truncated = -5 / 3;

    let remainder = 42 % 24;

    println!(
        "Printing the above opetations:{},{},{},{},{},{}",
        addition, substraction, multiplication, quotient, truncated, remainder
    );

    //boolean type
    let t = true;
    let f: bool = false;
    println!("Printing the booleans:{},{}", t, f);

    //character type
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    println!("Printing the char:{},{},{}", c, z, heart_eyed_cat);

    //Compound tye

    // the tuple type
    let tup = (200, 3.2, 1);
    let (m, n, o) = tup; //  pattern matching to destructure a tuple value
    println!("The value of m is:{m}");
    println!("The value of n is:{n}");
    println!("The value of o is:{o}");

    let f = (7, 8, 4);
    let seven = f.0; //can also access a tuple element directly by using a period followed by the index of the value we want to access
    let eight = f.1;
    let four = f.2;
    println!("Directly accessed tupels are:{},{},{}", seven, eight, four);

    //array
    let a = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("Array using `{:?}`: {:?}", "a", a);
    println!("Fitst element of array a is:{}", a[0]);
    println!("Second element of array a is:{}", a[1]);

    let at: [i32; 3] = [1, 2, 3];

    let asm = [3; 6]; //this array contains 3 six times

    println!("Please enter an array index");

    let mut index_user = String::new();

    io::stdin()
        .read_line(&mut index_user)
        .expect("Failed to read line");

    let index_user: usize = index_user
        .trim()
        .parse()
        .expect("Index entered was not a  number");

    let element = asm[index_user];
    let element2 = at[index_user];

    println!("The value of the element at index {index_user} of asm is: {element}");
    println!("The value of the element at index {index_user} of at is: {element2}");
}
