#[derive(Debug)]
struct Rectangle1{
    width: u32,
    height: u32
}

fn main() {
    // version 1 - two variables
    {
        let width1 = 30;
        let height1 = 50;

        println!("Version 1 = {}", area_1(width1, height1));
    }

    // version 2 - tuple
    {
        let dimensions = (30, 50);

        println!("Version 2 = {}", area_2(dimensions));
    }

    // version 3 - struct ver.1
    {
        let rectangle = Rectangle1{
            width: 30,
            height: 50
        };

        println!("Version 3 = {}", area_3(& rectangle));
    }

    // printing Rectangle
    {
        let rectangle = Rectangle1{
            width: 30,
            height: 50
        };

        // Doesn't work because rectangle hasn't Display trait
        // println!("{}", rectangle); 

        // But with added Debug trait we could print it like this:
        // !!! println! print to stdout
        println!("Rectangle: {rectangle:?}");  // or {:#?} for more beautiful output

        // !!! dbg! print to stderr
        dbg!(&rectangle);
    }

    // One more interesting example of usage dbg!
    {
        let scale = 2;
        let rect1 = Rectangle1 {
            width: dbg!(30 * scale),
            height: 50,
        };

        dbg!(&rect1);
    }
}

fn area_1(width: u32, height: u32) -> u32 {
    width * height
}

fn area_2(dimensions : (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_3(rectangle: & Rectangle1) -> u32 {
    rectangle.width * rectangle.height
}