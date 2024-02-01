fn main() {
    let s1 = String::from("John");
    let s2 = s1.clone(); // if we give s1 directly we can't use s1 for more now
    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s = String::from("hello");
    takes_ownership(s);

    let x = 8;
    makes_copy(x);

    let s3 = gives_ownership();

    let s4 = String::from("hello");

    let s5 = takes_and_gives_back(s4);

    let s7 = String::from("Your lips my lips apocalypse");

    let (s6, len) = calculate_length(s7);

    println!("The length of '{}' is {}.", s6, len)
}

fn calculate_length(s4: String) -> (String, usize) {
    let length = s4.len();
    (s4, length)
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn gives_ownership() -> String {
    let some_string = String::from("Yours");
    some_string
}

fn makes_copy(x: i32) {
    println!("{}", x)
}

fn takes_ownership(s: String) {
    println!("{}", s)
}
