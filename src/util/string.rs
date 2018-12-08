use std::collections::HashMap;

pub fn count_letters(s: String) -> HashMap<char, i32> {
    let mut map: HashMap<char, i32> = HashMap::new();

    for c in s.chars().into_iter() {
        *map.entry(c).or_insert(0) += 1;
    }

    map
}
