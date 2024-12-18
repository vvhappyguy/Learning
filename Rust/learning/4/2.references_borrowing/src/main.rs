/**
 * Rules of references:
 * - At any given time, you can have either one mutable reference or any number of immutable references.
 * - References must always be valid.
 */

fn main() {
    {
        let a = String::from("123");
        let size = calculate_length(& a);
        println!("Size of {a} is {size}.");
    }

    {
        let a = String::from("123");
        // modify_error(&a); // modifying immutable string
    }

    {
        let mut a = String::from("123");
        modify(&mut a); // modifying immutable string
    }

    {
        let mut s = String::from("123");

        let r1 = &mut s;
        // let r2 = &mut s; // allowed to have only one mut ref at one time
    }

    {
        let mut s = String::from("123");

        let r1 = &mut s;
        let r2 = &s;

        // println!("{}, {}", r1, r2); // impossible to have immutable ref if you have mutable
        // allowed to have only one mut ref at one time
    }

    {
        let mut s = String::from("123");

        {
            let r1 = &mut s;
        } // r1 goes out of scope here, so it's okay to have to mut ref
        
        let r2 = &s;
    }
    
    {
        let mut s = String::from("123");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{r1} and {r2}");

        let r3 = &mut s; // no problem
        println!("{r3}");
    }

    {
        let mut s = String::from("123");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{r1} and {r2}");

    }


}

// Impossible to make a dangle reference (return ref to value which was dropped)
// fn dangle() -> &String { // dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String
//     &s // we return a reference to the String, s
// }

fn calculate_length(s: &String) -> usize {
    s.len()
}

// modifying immutable string
// fn modify_error(s: &String) {
//     s.push_str("a");
// }

fn modify(s: &mut String) {
    s.push_str("a");
}
    


