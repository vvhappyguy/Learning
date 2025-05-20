fn create_vec(len: i32, min_rnd: i32, max_rnd: i32) -> Vec<i32>
{
    use rand::Rng;
    
    let mut vec: Vec<i32> = Vec::new();
    for n in 0..len {
        let value = rand::thread_rng().gen_range(min_rnd..=max_rnd);
        vec.push(value);
    }
    vec.sort();
    vec
}

fn print_vec(v: &Vec<i32>)
{
    let len = v.len();
    println!("len is {len}");
    for i in v {
        print!("{i}, ");
    }
    print!("\n");
}

fn get_median_value(v: &Vec<i32>) -> i32
{
    let len = v.len() / 2;
    v[len]
}

fn get_mod_value(v: &Vec<i32>) -> i32
{
    use std::collections::HashMap;
    
    let mut values = HashMap::new();

    for i in v {
        let count = values.entry(i).or_insert(0);
        *count += 1;
    }

    let mut max = 0;
    let mut max_value = 0;
    for (key, value) in values.into_iter() {
        if value > max_value {
            max_value = value;
            max = *key;
        }
    }
    max
}

fn main() {
    let vec = create_vec(5, 1, 5);
    print_vec(&vec);
    let median = get_median_value(&vec);
    println!("median is {median}");
    let modv = get_mod_value(&vec);
    println!("mod is {modv}");

    let vec = create_vec(6, 1, 3);
    print_vec(&vec);
    let median = get_median_value(&vec);
    println!("median is {median}");
    let modv = get_mod_value(&vec);
    println!("mod is {modv}");
}
