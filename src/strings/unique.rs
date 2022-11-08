pub mod solution {
    use std::collections::HashMap;

    pub fn unique_chars(input: &str) -> bool {
        // simple solution is to
        // iterate thru the input
        // put each char into a map
        // error if the char is in the map
        let mut seen = HashMap::new();
        for c in input.chars() {
            let v = seen.get(&c);
            if let Some(_x) = v {
                return false;
            }
            seen.insert(c, true);
        }
        return true;
    }

    pub fn unique_no_map(input: &str) -> bool {
        for (i, c) in input.chars().enumerate() {
            for (j, d) in input.chars().enumerate() {
                if j <= i {
                    continue;
                }
                if c == d {
                    return false;
                }
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    use super::solution::*;

    #[test]
    fn test_unique_chars() {
        assert!(unique_chars(&String::from("abcdefg")));
        assert!(!unique_chars(&String::from("abca")));
    }
    #[test]
    fn test_unique_no_map() {
        assert!(unique_no_map(&String::from("abcdefg")));
        assert!(!unique_no_map(&String::from("abca")));
    }
}
