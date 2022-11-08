use ndarray::{arr2, Array1, Array2};
use num_traits::Num;

fn zeroout<T: num_traits::Num + Clone>(m: &mut Array2<T>) {
    let mut rows: Vec<usize> = Vec::new();
    let mut colums: Vec<usize> = Vec::new();
    let mut row_idx = 0;

    let d = m.raw_dim();
    for row in m.rows() {
        for (c, v) in row.indexed_iter() {
            if *v == T::zero() {
                rows.push(row_idx);
                colums.push(c);
            }
        }
        row_idx += 1;
    }

    let zero_row: Array1<T> = Array1::zeros(d[1]);

    let zero_col: Array1<T> = Array1::zeros(d[0]);

    for r in rows {
        m.row_mut(r).assign(&zero_row);
    }
    for c in colums {
        m.column_mut(c).assign(&zero_col);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zeroout() {
        let mut a: Array2<i32> = arr2(&[[0, 1, 2], [4, 5, 6]]);
        zeroout(&mut a);
        assert_eq!(a, arr2(&[[0, 0, 0], [0, 5, 6]]));
        a = arr2(&[[10, 1, 2], [4, 5, 6]]);
        zeroout(&mut a);
        assert_eq!(a, arr2(&[[10, 1, 2], [4, 5, 6]]));
        a = arr2(&[[10, 1, 2], [4, 0, 6]]);
        zeroout(&mut a);
        assert_eq!(a, arr2(&[[10, 0, 2], [0, 0, 0]]));

        let mut f: Array2<f32> = arr2(&[[0., 1., 2.], [4., 5., 6.]]);
        zeroout(&mut f);
        assert_eq!(f, arr2(&[[0., 0., 0.], [0., 5., 6.]]));
    }
}
