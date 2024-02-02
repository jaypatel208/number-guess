fn main() {
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("Yo yo wassup??");

    change(&mut s2);

    let b1 = &mut s2;
    //  let b2 = &mut s2; //cannot borrow `s2` as mutable more than once at a time , but we can do by use it in something like a scope
    println!("{}", b1);

    // creating different scopes for multiple mutable references
    let mut s3 = String::from("Dil hai ye sochta fir bhi nahi pata..");
    {
        let b2 = &mut s3;
        println!("{}", b2);
    }
    let b3 = &mut s3;
    println!("{}", b3);

    //combining mutable and immutable references will give an error, that's why the below approach needs to be taken
    let mut s4 = String::from("Tere sang yaara....");

    let t1 = &s4;
    let t2 = &s4;
    println!("{} oo {}", t1, t2);
    // here t1 and t2 will not be used after this point

    let t3 = &mut s4;
    println!("{}", t3);
}

fn change(s2: &mut String) {
    s2.push_str(", All good!!");
    println!("{}", s2);
}

fn calculate_length(s1: &str) -> usize {
    s1.len()
}
