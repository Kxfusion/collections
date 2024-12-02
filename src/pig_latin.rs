pub fn pig_latinize(mut regular: String) -> String {
    let mut u8_range = [0; 4];
    
    let first_letter = match regular.chars().nth(0) {
        None => "",
        Some(letter) => letter.encode_utf8(&mut u8_range),
    };

    if ["a", "e", "i", "o", "u"].contains(&first_letter) {
        regular + "hay"
    } else {
        regular.split_off(1) + first_letter + "ay"
    }
}