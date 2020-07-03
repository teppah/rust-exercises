use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let apple = to_pig_latin("apple");
    println!("{}", apple);
    let cons = to_pig_latin("table");
    println!("{}", cons);
}


fn to_pig_latin(s: &str) -> String {
    let mut str = String::new();
    let first_char = s.chars().next().expect("you sent an empty string");
    if is_vowel(first_char) {
        str.push_str(s);
        str.push_str("-hay");
    } else {
        let mut iter = s.chars().into_iter();
        iter.next();
        let new_str = String::from_iter(iter);
        str.push_str(&new_str);
        str.push_str("-");
        str.push(first_char);
        str.push_str("ay");
    }
    return str;
}

fn is_vowel(c: char) -> bool {
    let mut vowels = HashSet::new();
    vowels.insert('e');
    vowels.insert('i');
    vowels.insert('o');
    vowels.insert('u');
    vowels.insert('a');
    vowels.contains(&c)
}

