fn main() {
    println!("Hello, world!");

    another_function();

    another_function_with_param(10);

    func1();

    let five = five();
    println!("The value of x is: {five}");
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_param(x: i32) {
    println!("Another function with param {x}.");
}

fn five() -> i32 {
    5
}

fn func1() {
    let y = { // statement;
        let x = 3;
        x + 1 // expression
    };

    println!("The value of y is: {y}");
}