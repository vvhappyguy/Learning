fn main() {
    {
        let v: Vec<i32> = Vec::new();
        let v = vec![1, 2, 3];
    }

    {
        let mut v = Vec::new();

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    }

    {
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        println!("The third element is {third}");

        let third: Option<&i32> = v.get(2);
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element."),
        }
    }

    {
        let v = vec![1, 2, 3, 4, 5];

        // index out of bounds: the len is 5 but the index is 100
        // let does_not_exist = &v[100];
        // let does_not_exist = v.get(100);
    }

    {
        let mut v = vec![1, 2, 3, 4, 5];

        let first = &v[0];

        // immutable borrow occurs here
        // v.push(6);

        println!("The first element is: {first}");
    }

    {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{i}");
        }
    }

    {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
    }

    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }

    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
    
}
