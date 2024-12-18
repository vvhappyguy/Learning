fn main() {
    f_to_c_tests();
    fib_tests();
    song_lyrics();
}

// The Twelve Days of Christmas Lyrics
fn song_lyrics()
{
    let str1 = "On the first day of Christmas, my true love sent to me";
    let str2 = "Five golden rings";
    let str3 = "Four calling birds";
    let str4 = "Three french hens";
    let str5 = "Two turtle doves and";
    let str6 = "A partridge in a pear tree";

    let verses_amount = 5;
    let mut counter = 0;
    while verses_amount > counter {
        
        println!("\n{str1}");

        if counter > 3 {
            println!("{str2}");
        }

        if counter > 2 {
            println!("{str3}");
        }

        if counter > 1 {
            println!("{str4}");
        }

        if counter > 0 {
            println!("{str5}");
        }

        println!("{str6}");
        counter = counter + 1;
    }
}

fn fib_tests()
{
    let n = 0;
    let number = fib(n);
    println!("{n}-th fibonacci number is {number}");

    let n = 1;
    let number = fib(n);
    println!("{n}-th fibonacci number is {number}");

    let n = 2;
    let number = fib(n);
    println!("{n}-th fibonacci number is {number}");

    let n = 3;
    let number = fib(n);
    println!("{n}-th fibonacci number is {number}");

    let n = 6;
    let number = fib(n);
    println!("{n}-th fibonacci number is {number}");

    let n = 10;
    let number = fib(n);
    println!("{n}-th fibonacci number is {number}");
}

fn fib(n: usize) -> usize
{
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut h_num = 0;
    let mut num = 1;

    let mut counter = n - 1;
    while counter > 0 {
        let temp = h_num;
        h_num = num;
        num = temp + h_num;
        counter = counter - 1;
    }

    return num;
}

fn f_to_c_tests()
{
    // Fahrenheit to Celsius and back
    let temp_c = 25.0;
    println!("Temperature in Celsius: {temp_c}");
    let temp_f = C_to_F(temp_c);
    println!("Temperature in Celsius: {temp_f}");
    let temp_c = F_to_C(temp_f);
    println!("Temperature in Celsius: {temp_c}");

    let temp_c = -25.0;
    println!("Temperature in Celsius: {temp_c}");
    let temp_f = C_to_F(temp_c);
    println!("Temperature in Celsius: {temp_f}");
    let temp_c = F_to_C(temp_f);
    println!("Temperature in Celsius: {temp_c}");
}

fn F_to_C(value: f32) -> f32 {
    (value - 32.0) * 5.0 / 9.0
}

fn C_to_F(value: f32) -> f32 {
    value * 9.0 / 5.0 + 32.0
}


