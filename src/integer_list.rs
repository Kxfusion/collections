use std::collections::HashMap;

pub fn median(mut list: Vec<i32>) -> i32 {
    if list.is_empty() {
        0
    }

    else if list.len() == 1 {
        match list.get(0) {
            Some(num) => *num,
            None => 0
        }
    }

    else {
        list.sort_unstable();
        let index = (list.len() - 1) / 2;

        if list.len() % 2 == 0 {
            let lower = list[index];
            let higher = list[index + 1];
            (lower + higher) / 2

        } else {
            list[index]
        }
    }
}

pub fn mode(list: &Vec<i32>) -> i32 {
    let mut int_count = HashMap::new();
    for int in list {
        let count = int_count.entry(int).or_insert(0);
        *count += 1;
    }

    match int_count.into_iter().max_by(|x, y| x.1.cmp(&y.1)) {
        None => 0,
        Some(item) => *item.0,
    }
}
