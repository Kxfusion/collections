use std::{collections::HashMap, error::Error};
mod integer_list;
mod pig_latin;

fn main() -> Result<(), Box<dyn Error>> {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(8);
    v.push(5);
    v.push(10);
    v.push(9);
    v.push(9);
    v.push(2);
    v.push(6);
    v.push(9);

    let third = &v[2];
    println!("{third}");

    for i in &v {
        println!("{i}")
    }

    let third = v.get(2);
    match third {
        Some(third) => println!("{third}"),
        None => println!("X"),
    }

    // for i in &mut v {
    //     *i += 50;
    // }

    println!("{v:?}");

    let mode = integer_list::mode(&v);
    let median = integer_list::median(v);

    println!("{mode}");
    println!("{median}");

    let s = "stuff".to_string();

    println!("{s}");

    let mut s = String::new();

    s.push_str("stuffing");

    println!("{s}");

    let s2 = String::from("me");

    let s3 = s + " " + &s2;
    let s = format!("{s2} {s3}");

    println!("{s}");

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

    let piggy1 = pig_latin::pig_latinize(String::from("apple"));
    let piggy2 = pig_latin::pig_latinize(String::from("snapple"));

    println!("{piggy1} {piggy2}");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20);
    scores.insert(String::from("Yellow"), 50);
    scores.entry(String::from("Green")).or_insert(35);
    scores.entry(String::from("Blue")).or_insert(55);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let some_text = "La La Blah Nah";

    let mut map = HashMap::new();

    for word in some_text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

    Ok(())
}
