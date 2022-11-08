fn run_length_encoding(s: &str) -> String {
    let mut cnt = 1;
    let mut prev: char = ' ';

    let mut output = String::new();
    for (i, c) in s.chars().enumerate() {
        if i == 0 {
            prev = c;
            continue;
        } else {
            if c == prev {
                cnt += 1;
            } else {
                output.push(prev);
                output.push_str(&cnt.to_string());
                cnt = 1;
                prev = c;
            }
        }
        if cnt == 1 {
            output.push(prev);
            output.push_str(&cnt.to_string());
        }
    }
    if output.len() < s.len() {
        return output;
    }
    s.to_string()
}

mod tests {
    use super::*;

    #[test]
    fn test_rle() {
        let a = String::from("aaaaab");
        let b = String::from("abc");
        assert_eq!(run_length_encoding(&a), String::from("a5b1"));
        assert_eq!(run_length_encoding(&b), b);
    }
}
