// Using a hash map and vectors, 
// create a text interface to allow a user to add employee names to a department in a company; 
// for example, “Add Sally to Engineering” or “Add Amir to Sales.” 
// Then let the user retrieve a list of all people in a department or 
// all people in the company by department, sorted alphabetically.

use std::io;
use std::collections::HashMap;

#[derive(Debug)]
enum Command {
    Add { name: String, department: String },
    ListByOneDepartment {department: String},
    ListAll,
    Help,
    Unexpected,
}

type DB = HashMap<String, Vec<String>>;

impl Command {
    fn execute(&self, db: &mut DB) {
         match self {
            Command::Help => {
                let help = 
                "
                > help - for help
                > add {Name} department {Department} - to add people to department
                > listByDepartment {Department} - list all people in department
                > listAll - list all people in all department sorted alphabetically
                ";
                println!("{help}");
            },
            Command::Add {name, department} => {
                db.entry(department.to_string())
                .or_default()
                .push(name.to_string());
            },
            Command::ListByOneDepartment {department} => {
                match db.get(department) {
                    None => {println!("No department {department}");},
                    Some(_) => {
                        for name in db.get(department).unwrap() {
                            print!("{name},  ")
                        }
                        println!("");
                    },
                }          
            },
            Command::ListAll => {
                for vec in db.values_mut() {
                    vec.sort();
                }

                let mut sorted_entries: Vec<_> = db.into_iter().collect();
                sorted_entries.sort_by(|a, b| a.0.cmp(&b.0));
                for (key, value) in sorted_entries {
                    println!("{key}:");
                    print!("  ");
                    for name in value {
                            print!("{name}, ");
                    } 
                    println!("");
                }
            }
            _ => {
                println!("try `help` command");
                println!("{self:?}");
            }
         }

    }
}

fn parse_input(input: &str) -> Command
{
    let words = input.split_whitespace();
    let mut words_vec = Vec::new();
    for word in words {
        words_vec.push(word.to_string());
    }

    match words_vec.len() {
        1 => {
            println!("1");
            match words_vec[0].as_str() {
                "help" => Command::Help,
                "listAll" => Command::ListAll,
                _ => Command::Unexpected
            }
        },
        2 => {
            println!("2");
            match words_vec[0].as_str() {
                "listByDepartment" => Command::ListByOneDepartment { department: words_vec[1].clone()},
                _ => Command::Unexpected
            }
        },
        4 => Command::Add {name: words_vec[1].clone(), department: words_vec[3].clone()},
        _ => Command::Unexpected

    }
}

fn main() {
    let mut db : DB = HashMap::new();
    db.insert(String::from("A"), vec!["Ivan".to_string()]);

    loop {
        print!("> ");

        let mut input = String::new();
        
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");        

        let command = parse_input(&input);
        command.execute(&mut db);

    }
}
