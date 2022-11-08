// take list of pairs of sorted numbers and return merged intervals
/*
use ndarray::{Array2, Array1};

fn max<'a,T:PartialOrd>(first: &'a T, second: &'a T) -> &'a T {
    if first > second {
        return first
    }
    return second
}
fn  merge<T: num_traits::Num + PartialOrd +Clone>(intervals: &Array2<T>) -> &Array2<T> {
    let mut t = Vec::<Array1<T>>::new();
    let mut idx =0;
    t.push(Array1::from_vec(intervals.row(0).to_vec()));// .to_vec());
    for row in intervals.rows() {
        assert_eq!(row.len(),2);
        //non overlapping
        if row[0] > t[idx][1] {
            idx +=1;
            t[idx] = Array1::from_vec(row.to_vec());
        } else {
            if row[1] > t[idx][1] {
                t[idx][1] = row[1]
            }
            //t[idx][1] = max(&row[1],&t[idx][1]);
        }

    }
    return &Array2::from(&t) //from_rows(t[..idx])
}

#[cfg(test)]
mod tests {
    use ndarray::arr2;

    use super::*;

    #[test]
    fn test_merge_intervals() {
        let input = vec![vec![1,5], vec![2,9], vec![15,16]];
        let result = merge(input);
        assert_eq!(result, arr2(&[[1,9], [15, 16]]))
    }
}
*/
