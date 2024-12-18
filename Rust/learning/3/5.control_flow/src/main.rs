fn main() {
    if_example1();
    if_example2();
    loop_example1();
    loop_example2();
}

fn if_example1() {
    let a = 5;
    if a < 5 {
        println!("a is less than 5");
    } else {
        println!("a is equal or more than 5");
    }

    // Doesn't work in Rust
    // if a {
    //     println!("a is less than 5");
    // }
}

fn if_example2() {
    // if in statements
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // Couldn't use different types
    // let number = if condition { 5 } else { "six" };
}

fn loop_example1() {
    // loop
    loop {
        println!("again!");
        break;
    }

    // return value from loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // break to label
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
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
    println!("End count = {count}");
}

fn loop_example2() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
}
