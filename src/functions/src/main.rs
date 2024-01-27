fn main() {
    println!("Hello, world!");
    another_function();
    yet_another_function(45);
    print_labeled_measurments(5, 'h');

    // let x = (let y =6);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is:{}", y);

    let z = five();
    println!("The value of z is:{}", z);

    let p = plus_one(5);
    println!("Five plus one is: {}", p)
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // x + 1; // a semicolon at the end of the line changes it from an expression to a statement, we’ll get an error
}

fn print_labeled_measurments(value: i32, unit_label: char) {
    println!("The measurment is:{value}{unit_label}");
}

fn yet_another_function(x: i32) {
    println!("The value of x is: {}", x)
}

fn another_function() {
    println!("Another function.")
}

fn five() -> i32 {
    5
}
