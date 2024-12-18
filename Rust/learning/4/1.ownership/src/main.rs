/**
 * Rules:
 * - Each value in Rust has an owner.
 * - There can only be one owner at a time.
 * - When the owner goes out of scope, the value will be dropped.
 */

fn main() {
    {
        let s = "hello"; // stack static string

        // can't be mutated
    }
    
    {
        let mut s = String::from("hello"); // heap string

        s.push_str(", world!"); // push_str() appends a literal to a String

        println!("{s}"); // This will print `hello, world!`
    } // drop() for variable `s` called here!


    // Ownership depends on data type
    // For scalars it's okay
    {
        let x = 5;
        let y = x;

        println!("{x}");
    }

    
    {
        let s1 = String::from("hello");
        let s2 = s1;

        // But for heap string doesn't
        // error[E0382]: borrow of moved value: `s1`
        // println!("{s1}");
    }

    {
        let mut s = String::from("hello");
        s = String::from("ahoy"); // drop for previous value called here
    
        println!("{s}, world!");
    }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        // Data cloned on the heap
        println!("s1 = {s1}, s2 = {s2}");
    }
    
    // Copy trait annotation means all data could be stored on the stack
    // i64, bool, char, f64, (i32, f64) - ok, (i32, String) - not okay
    // String implemented Drop trait

    {
        // ownership
        let s = String::from("hello");
        takes_ownership(s);

        // copy
        let x:i32 = 5;
        makes_copy(x);
    }
    
    {
        let s1 = gives_ownership();
        let s2 = String::from("123");
        let s3 = takes_and_gives_back(s2);
    }

    {
        let s1 = String::from("123");
        let (s2, len) = calculate_length(s1);
        println!("Len of {s2} is {len}");
    }
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn gives_ownership() -> String {
    let s = String::from("hello!");
    s
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}
