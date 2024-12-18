fn main() {
    {
        let mut s = String::from("123");
        let s_len = first_word_1(&s);

        println!("s_len = {s_len}");

        s.clear();

        // now s_len is invalid, because s now is ""
    }

    {
        // Slices!
        let s = String::from("hello world");

        let hello = &s[0..5]; // end_index = start + end - start (5 is not included)
        let world = &s[6..11];

        println!("{hello}");
        println!("{world}");

        let hello = &s[4..]; // means 4..len
        let world = &s[..6]; // means 0..6

        println!("{hello}");
        println!("{world}");

        let world = &s[..]; // means 0..len
        println!("{world}");
    }

    {
        let mut s = String::from("123");
        let s1 = first_word_2(&s);

        println!("s1 = {s1}");

        s.clear();

        // using mutable after immutable
        // println!("s1 = {s1}");
    }

    {
        let s_literal = "Hello, world!"; // type is &str
        // let s2 = first_word_2(&s_literal); 
        // we need a version which takes slices

        let s2 = first_word_3(&s_literal);
        println!("s2 = {s2}"); // Hello,

        // also we could pass slices of String or str to it
        let s2 = first_word_3(&s_literal[1..4]);
        println!("s2 = {s2}"); // Hello,

        let s_literal = String::from("Hello, world!"); // type is &str
        let s2 = first_word_3(&s_literal[1..4]);
        println!("s2 = {s2}"); // Hello,
     }

     {
        // Also with arrays
        let a = [1, 2, 3, 4, 5];

        let slice = &a[1..3];

        assert_eq!(slice, &[2, 3]);
     }
}

fn first_word_3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_1(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}