use std::collections::HashMap;

pub enum Mode {
    Map,
    Sort,
}

pub fn is_permutation(a: &str, b: &str, m: Mode) -> bool {
    if a.len() != b.len() {
        return false;
    }
    if let Mode::Map = m {
        return perm_via_map(a, b);
    } else {
        let mut z: String = String::new();
        z.clone_from(&a.to_string());
        let mut y: String = String::new();
        y.clone_from(&b.to_string());
        perm_via_sort(&mut z, &mut y)
    }
}
fn perm_via_sort(a: &mut str, b: &mut str) -> bool {
    use crate::sorting::mergesort;
    let mut a_chars = Vec::new();
    for c in a.chars() {
        a_chars.push(c);
    }
    let mut b_chars = Vec::new();
    for d in b.chars() {
        b_chars.push(d);
    }

    println!("a {:?}. b {:?}", a_chars, b_chars);
    let a_slc = a_chars.as_mut_slice();
    let b_slc = b_chars.as_mut_slice();
    mergesort::sort(a_slc);
    mergesort::sort(b_slc);
    println!("aslc {:?}. bslc {:?}", a_slc, b_slc);
    for (i, b) in b_slc.iter().enumerate() {
        if let Some(a) = a_slc.get(i) {
            if a != b {
                return false;
            }
        }
    }
    return true;
}
fn perm_via_map(a: &str, b: &str) -> bool {
    //  <&str as IntoIterator>::Item
    let map_a: HashMap<char, i32> = HashMap::new();
    let map_b: HashMap<char, i32> = HashMap::new();
    struct StringCounter<'a> {
        input: &'a str,
        char_count: HashMap<char, i32>,
    }
    let mut str_cnt_a = StringCounter {
        input: a,
        char_count: map_a,
    };
    let mut str_cnt_b = StringCounter {
        input: b,
        char_count: map_b,
    };
    let cntrs = [&mut str_cnt_a, &mut str_cnt_b];
    for cntr in cntrs {
        for elem in cntr.input.chars() {
            let cnt = cntr.char_count.get_mut(&elem);
            match cnt {
                Some(v) => *v += 1,
                None => {
                    cntr.char_count.insert(elem, 1);
                }
            }
        }
    }
    for (k_a, v_a) in str_cnt_a.char_count {
        let entry = str_cnt_b.char_count.get_key_value(&k_a);
        if entry != Some((&k_a, &v_a)) {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perm() {
        let a = String::from("mynameis");
        let b = String::from("nameisym");
        assert!(is_permutation(&a, &b, Mode::Map));
        assert!(!is_permutation(&a, &String::from("mymameis"), Mode::Map));
        assert!(is_permutation(&a, &b, Mode::Sort));
        assert!(!is_permutation(&a, &String::from("mymameis"), Mode::Sort));
    }
}
