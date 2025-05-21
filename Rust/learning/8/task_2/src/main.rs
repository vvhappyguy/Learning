// Convert strings to pig latin. 
// The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. 
// Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). 
// Keep in mind the details about UTF-8 encoding!

fn is_vowel(c: char) -> bool
{
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    for v in vowels {
        if c == v {
            return true;
        }
    }
    return false;
}

fn convert_to_pig_latin_as_vowel(string: &str) -> String
{
    let mut result_string = String::new();
    result_string.push_str(string);
    result_string.push_str("-hay");
    result_string
}

fn convert_to_pig_latin_not_as_vowel(string: &str) -> String
{
    let mut result_string = String::new();
    let first_letter = string.chars().next().unwrap();
    result_string.push_str(&string[1..]);
    result_string.push('-');
    result_string.push(first_letter);
    result_string.push_str("ay");
    result_string
}

fn convert_to_pig_latin(string: &str) -> String
{
    println!("{string}");
    let first_letter = string.chars().next().unwrap();
    
    if is_vowel(first_letter) {
        convert_to_pig_latin_as_vowel(string)
    } else {
        convert_to_pig_latin_not_as_vowel(string)
    }
}

fn main() {
    let mut strings: Vec<String> = Vec::new();
    strings.push(String::from("first"));
    strings.push(String::from("apple"));
    strings.push(String::from("boy"));
    strings.push(String::from("idea"));

    for string in strings {
        let converted_string = convert_to_pig_latin(&string);
        println!("{converted_string}");
    }
    
}
