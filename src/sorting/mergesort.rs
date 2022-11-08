pub fn sort<T: Copy + std::cmp::PartialOrd>(list: &mut [T]) {
    return merge_sort(list);
}

fn merge_sort<T: Copy + PartialOrd>(list: &mut [T]) {
    let mid = list.len() / 2;
    let end = list.len();

    if end <= 1 {
        return;
    }

    merge_sort(&mut list[0..mid]);
    merge_sort(&mut list[mid..end]);
    let mut tmp = list.to_vec();
    merge(&list[0..mid], &list[mid..end], &mut tmp);

    list.copy_from_slice(tmp.as_slice())
}
fn merge<T: Copy + std::cmp::PartialOrd>(
    sorted_list_a: &[T],
    sorted_list_b: &[T],
    merged: &mut [T],
) {
    let mut idx = 0;
    let mut a_cursor = 0;
    let mut b_cursor = 0;
    while idx < merged.len() {
        if a_cursor >= sorted_list_a.len() {
            merged[idx] = sorted_list_b[b_cursor];
            b_cursor += 1;
            idx += 1;
            continue;
        }

        if b_cursor >= sorted_list_b.len() {
            merged[idx] = sorted_list_a[a_cursor];
            a_cursor += 1;
            idx += 1;
            continue;
        }

        let a = sorted_list_a[a_cursor];
        let b = sorted_list_b[b_cursor];
        if a < b {
            merged[idx] = sorted_list_a[a_cursor];
            a_cursor += 1
        } else {
            merged[idx] = sorted_list_b[b_cursor];
            b_cursor += 1
        }
        idx += 1
    }
    return;
}

fn sorted_merge<T: Copy + PartialOrd>(a: &mut [T], len: usize, b: &[T]) {
    assert!(a.len() >= (len + b.len()));
    let mut a_idx = len - 1;
    let mut b_idx = b.len() - 1;
    let mut sorted_idx = len + b.len() - 1;
    while sorted_idx > 0 {
        if a[a_idx] > b[b_idx] {
            a[sorted_idx] = a[a_idx];
            if a_idx > 0 {
                a_idx -= 1;
            }
        } else {
            a[sorted_idx] = b[b_idx];
            if b_idx > 0 {
                b_idx -= 1;
            }
        }
        sorted_idx -= 1;
    }
}
#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_merge() {
        let input1 = vec![1, 3];
        let input2 = vec![2, 4, 6, 8];

        let mut output = Vec::with_capacity(input1.len() + input2.len());
        output.resize(output.capacity(), 0);
        merge(&input1, &input2, &mut output);
        assert_eq!(output, vec![1, 2, 3, 4, 6, 8])
    }

    #[test]
    fn test_merge_sort() {
        let mut list = vec![1, 4, 3, 8, 6, 2];

        merge_sort(&mut list);
        assert_eq!(list, vec![1, 2, 3, 4, 6, 8])
    }

    #[test]
    fn test_sorted_merge() {
        let mut a = vec![1, 4, 10];
        a.resize_with(6, Default::default);
        assert_eq!(6, a.len());
        let b = vec![3, 5, 7];

        sorted_merge(&mut a, 3, &b);
        assert_eq!(a, [1, 3, 4, 5, 7, 10])
    }
}
