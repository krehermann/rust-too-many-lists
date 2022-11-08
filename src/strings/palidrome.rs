use std::collections::HashMap;

fn is_palidrome_perm(s: &str) -> bool {
    let mut char_map: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        if let Some(v) = &char_map.get(&c) {
            char_map.insert(c, *v + 1);
        } else {
            char_map.insert(c, 1);
        }
    }
    let mut odd_cnt = 0;
    for (_, v) in char_map {
        if (v % 2) != 0 {
            odd_cnt += 1;
        }
    }
    return (odd_cnt == 1 && s.len() % 2 != 0) || odd_cnt == 0;
    // return true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palidrome_perm() {
        let a = String::from("mmeei");
        let b = String::from("mmeeir");
        assert!(is_palidrome_perm(&a));
        assert!(!is_palidrome_perm(&b));
    }
}
