fn main() {

    // borrowed value does not live long enough
    // {
    //     let r;

    //     {
    //         let x = 5;
    //         r = &x;
    //     }
    //     

    //     println!("r: {r}");
    // }


    {
        let x = 5;          // ----------+-- 'b
                            //           |
        let r = &x;         // --+-- 'a  |
                            //   |       |
        println!("r: {r}");
    }

    // ^ expected named lifetime parameter
    // {
    //     fn longest(x: &str, y: &str) -> &str {
    //         if x.len() > y.len() { x } else { y }
    //     }

    //     let string1 = String::from("abcd");
    //     let string2 = "xyz";

    //     let result = longest(string1.as_str(), string2);
    //     println!("The longest string is {result}");
    // }

    {
        // &i32;        // a reference
        // &'a i32;     // a reference with an explicit lifetime
        // &'a mut i32; // a mutable reference with an explicit lifetime
    }

    {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() { x } else { y }
        }

        {
            let string1 = String::from("long string is long");

            {
                let string2 = String::from("xyz");
                let result = longest(string1.as_str(), string2.as_str());
                println!("The longest string is {result}");
            }
        }

        // string2 ^^^^^^^ borrowed value does not live long enough
        // {
        //     let string1 = String::from("long string is long");
        //     let result;
        //     {
        //         let string2 = String::from("xyz");
        //         result = longest(string1.as_str(), string2.as_str());
        //     }
        //     println!("The longest string is {result}");
        // }   
    }

    {
        fn longest<'a>(x: &'a str, y: &str) -> &'a str {
            x
        }
    }

    {
        // returns a value referencing data owned by the current function
        // fn longest<'a>(x: &str, y: &str) -> &'a str {
        //     let result = String::from("really long string");
        //     result.as_str()
        // }
    }

    {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().unwrap();
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }

    {
        use std::fmt::Display;

        fn longest_with_an_announcement<'a, T>(
            x: &'a str,
            y: &'a str,
            ann: T,
        ) -> &'a str
        where
            T: Display,
        {
            println!("Announcement! {ann}");
            if x.len() > y.len() { x } else { y }
        }
    }
}