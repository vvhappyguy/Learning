#[derive(Debug)]
enum IpAddrKind
{
    V4,
    V6
}

fn route(ip :IpAddrKind)
{
    dbg!(ip);
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        dbg!(self);
    }
}

fn main() {
    {
        let _ip_v4 = IpAddrKind::V4;
        let _ip_v6 = IpAddrKind::V6;
        route(_ip_v4);
        route(_ip_v6);
    }

    {
        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));

        dbg!(home);
        dbg!(loopback);
    }

    {
        let m = Message::Write(String::from("hello"));
        m.call();
    }

    {
        let x: i8 = 10;
        let y: Option<i8> = Some(5);

        // error - different types
        // let sum = x + y;
    }
}
