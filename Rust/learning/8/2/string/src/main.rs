fn main() {
    
    {
        let mut s = String::new();
        println!("s is {s}");
    }

    {
        let data = "initial contents";

        let s = data.to_string();
        println!("s is {s}");

        // The method also works on a literal directly:
        let s = "initial contents".to_string();
        println!("s is {s}");

        let s = String::from("initial contents");
        println!("s is {s}");
    }

    // Power of utf-8
    {
        let hello = String::from("السلام عليكم");
        let hello = String::from("Dobrý den");
        let hello = String::from("Hello");
        let hello = String::from("שלום");
        let hello = String::from("नमस्ते");
        let hello = String::from("こんにちは");
        let hello = String::from("안녕하세요");
        let hello = String::from("你好");
        let hello = String::from("Olá");
        let hello = String::from("Здравствуйте");
        let hello = String::from("Hola");
    }

    {
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {s2}");
    }

    {
        let mut s = String::from("lo");
        s.push('l');
        println!("s is {s}");
    }

    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

        println!("s3 is {s3}");
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = s1 + "-" + &s2 + "-" + &s3;
        println!("s is {s}");
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{s1}-{s2}-{s3}");
        println!("s is {s}");
    }

    // Error - no strings indexing
    {
        // let s1 = String::from("hi");
        // let h = s1[0];
    }

    {
        // len = 4
        let hello = String::from("Hola");

        // len = 24, not 12, because utf-8
        // so it's not possible to get letter З
        let hello = String::from("Здравствуйте");

        let hello = "Здравствуйте";
        // let answer = &hello[0];
    }

    // three relevant ways to look at strings from Rust’s perspective: as bytes, scalar values, and grapheme clusters
    {
        // Hindi word “नमस्ते”
        // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]

        // ['न', 'म', 'स', '्', 'त', 'े']
        // ["न", "म", "स्", "ते"]
    }

    {
        let hello = "Здравствуйте";

        // Зд
        let s = &hello[0..4];

        // Зд
        for c in s.chars() {
            println!("{c}");
        }

        // 208
        // 151
        // 208
        // 180
        for b in s.bytes() {
            println!("{b}");
        }

        
    }
}
