fn main() {
    let number = 4;
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false")
    }

    if number != 0 {
        println!("Number was something other than zero")
    }

    if number % 4 == 0 {
        println!("Number is divisble by 4")
    } else if number % 3 == 0 {
        println!("Number is divisible by 3")
    } else if number % 2 == 0 {
        println!("Number is divisible by 2")
    } else {
        println!("Number is not divisible by 4, 3 or 2")
    }

    let condition = true;
    let number_by_if = if condition { 5 } else { 6 };

    println!("The value fron numbe if is:{}", number_by_if);

    returning_value_from_loop();

    loop_labels();

    while_loop();

    looping_through_collection();

    for_loop();

    for_loop_range();

    loop {
        println!("This print will not stop until we explicitly stop it.")
    }
}

fn for_loop_range() {
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF!");
}

fn for_loop() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

    for element in a {
        println!("The value is: {element}");
    }
}

fn looping_through_collection() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }
}

fn while_loop() {
    let mut number = 7;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF");
}

fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining ={remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
}

fn returning_value_from_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result)
}
