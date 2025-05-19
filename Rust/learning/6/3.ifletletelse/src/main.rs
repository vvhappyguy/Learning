#[derive (Debug)]
enum UsState
{
    Alabama,
    Alaska,
}

enum Coin1
{
    Quarter(UsState),
}

enum Coin
{
    Quarter(String),
}


fn main() {

    // if let ...
    {

        
        // Was
        {
            let config_max = Some(3u8);
            match config_max {
                Some(max) => println!("The maximum is configured to be {}", max),
                _ => (),
            }
        }

        // Now
        {
            let config_max = Some(4u8);
            if let Some(max) = config_max {
                println!("The maximum is configured to be {}", max);
            }
        }

        // Was
        {
            let coin = Coin::Quarter(String::from("California"));
            let mut count = 0;
            match coin {
                Coin::Quarter(state) => println!("State quarter from {state:?}!"),
                _ => count += 1,
            }
        }

        // Now
        {
            let coin = Coin::Quarter(String::from("Nevada"));
            let mut count = 0;
            if let Coin::Quarter(state) = coin {
                println!("State quarter from {state:?}!");
            } else {
                count += 1;
            }
        }
    }

    // let ... else
    {
        
        impl UsState {
            fn existed_in(&self, year: u16) -> bool {
                match self {
                    UsState::Alabama => year >= 1819,
                    UsState::Alaska => year >= 1959,
                }
            }
        }

        {
            fn describe_state_quarter(coin: Coin1) -> Option<String> {
                if let Coin1::Quarter(state) = coin {
                    if state.existed_in(1900) {
                        Some(format!("{state:?} is pretty old, for America!"))
                    } else {
                        Some(format!("{state:?} is relatively new."))
                    }
                } else {
                    None
                }
            }

            let coin = Coin1::Quarter(UsState::Alaska);
            let state_quarter = describe_state_quarter(coin);
            println!("{:?}", state_quarter);
        }

        {
            fn describe_state_quarter(coin: Coin1) -> Option<String> {
                let state = if let Coin1::Quarter(state) = coin {
                    state
                } else {
                    return None;
                };
            
                if state.existed_in(1900) {
                    Some(format!("{state:?} is pretty old, for America!"))
                } else {
                    Some(format!("{state:?} is relatively new."))
                }
            }

            let coin = Coin1::Quarter(UsState::Alabama);
            let state_quarter = describe_state_quarter(coin);
            println!("{:?}", state_quarter);
        }
    
        {
            fn describe_state_quarter(coin: Coin1) -> Option<String> {
                let Coin1::Quarter(state) = coin else {
                    return None;
                };
            
                if state.existed_in(1900) {
                    Some(format!("{state:?} is pretty old, for America!"))
                } else {
                    Some(format!("{state:?} is relatively new."))
                }
            }

            let coin = Coin1::Quarter(UsState::Alaska);
            let state_quarter = describe_state_quarter(coin);
            println!("{:?}", state_quarter);
        }
    }
}
