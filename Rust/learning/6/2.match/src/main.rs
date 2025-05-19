#[derive (Debug)]
enum DesyatCoin
{
    Default,
    Ubileyniy
}

enum Coin
{
    Rubl,
    DvaRublya,
    PyatRubley,
    DesyatRubley(DesyatCoin),
}

fn value_in_rubles(coin: Coin) -> u8 {
    match coin {
        Coin::Rubl =>
        {
            println!("Odin Rubl");
            1
        },
        Coin::DvaRublya => 2,
        Coin::PyatRubley => 5,
        Coin::DesyatRubley(type_coin) => {
            dbg!(type_coin);
            10
        },
    }
}

fn plus_one(value: Option<i32>) -> Option<i32> {
    match value {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn wait()
{

}

fn step()
{
    
}

fn step_back()
{
    
}

fn main() {
    {
        let value : u8 = value_in_rubles(Coin::Rubl);
        println!("{}", value);

        let value : u8 = value_in_rubles(Coin::DesyatRubley(DesyatCoin::Ubileyniy));
        println!("{}", value);
    }

    {
        let five = Some(5);
        let none = plus_one(None);
        let six = plus_one(five);
    }

    {
        let dice_roll = 9;
        match dice_roll {
            0 => wait(),
            1 => step(),
            2 => step_back(),
            other => println!("{}", other)
        }

        match dice_roll {
            0 => wait(),
            1 => step(),
            2 => step_back(),
            _ => println!("Not handled")
        }
    }
}
