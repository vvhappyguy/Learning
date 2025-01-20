fn main() {
    // Scalar
    {
        // Integers
        {
            let int8: i8 = -128;
            let uint8: u8 = 255;

            let int16: i16 = -1;
            let uint16: u16 = 1;

            let int16: i16 = -1;
            let uint16: u16 = 1;

            let int32: i32 = -1;
            let uint32: u32 = 1;

            let int64: i64 = -1;
            let uint64: u64 = 1;

            let int64: i128 = -1;
            let uint64: u128 = 1;

            let int64: isize = -1;
            let uint64: usize = 1;
        }

        // Formats
        {
            let dec: isize = -1_000;
            let hex: isize = 0xff;
            let oct: isize = 0o77;
            let bin: isize = 0b1111_1010;
            let byte: u8 = b'A';
        }

        // Overflow
        {
            let mut dec: i8 = 127;

            // release = -128
            // debug = panic!
            // dec = dec + 1;

            println!("dec = {dec}");
        }

        // overflow functions from std
        {
            let a = i8::MAX.wrapping_add(2);
            println!("{a}"); // -127 even in release

            assert_eq!((i8::MAX - 2).checked_add(1), Some(i8::MAX - 1));
            assert_eq!((i8::MAX - 2).checked_add(3), None);

            assert_eq!(5i8.overflowing_add(2), (7, false));
            assert_eq!(i8::MAX.overflowing_add(1), (i8::MIN, true));

            assert_eq!(100i8.saturating_add(1), 101);
            assert_eq!(i8::MAX.saturating_add(100), i8::MAX);
            assert_eq!(i8::MIN.saturating_add(-1), i8::MIN);
        }

        // floating types
        {
            let x = 2.0; // f64

            let y: f32 = 3.0; // f32
        }

        // boolean types
        {
            let t = true;

            let f: bool = false; // with explicit type annotation
        }

        // char types
        {
            let c = 'z';
            let z: char = 'ℤ'; // with explicit type annotation
            let heart_eyed_cat = '😻';
        }
    }

    // Compound
    {
        // Tuples
        {
            let tup: (i32, f64, u8) = (500, 6.4, 1);

            let (x, y, z) = tup;

            println!("The value of y is: {y}");

            let a: (i32, f64, u8) = (500, 6.4, 1);

            let five_hundred = a.0;

            let six_point_four = a.1;
        
            let one = a.2;

            // unit
            let unit = ();
        }

        // array
        {
            let a = [1, 2, 3, 4, 5];

            let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

            let b: [i32; 5] = [1, 2, 3, 4, 5];

            let a = [3; 5]; // equals to let a = [3, 3, 3, 3, 3];

            let first = a[0];
            let second = a[1];

            // a[10] runtime error
        }
    }
}
