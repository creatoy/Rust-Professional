/*
    Rotate Matrix 90 Degrees
    Given a 2D matrix, rotate it 90 degrees in place.
    You need to perform the rotation without using any additional matrix storage.

    You need to implement the function `rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>)`.
    The function should rotate the input matrix in place.

    Hint: Consider rotating the matrix layer by layer, starting from the outermost layer and working your way inward.
*/

use std::fmt::{self, Display, Formatter};

pub fn transpose(matrix: &mut Vec<Vec<i32>>) {
    let rows = matrix.len();
    let cols = matrix[0].len();
    for i in 0..rows {
        for j in i + 1..cols {
            matrix[i][j] += matrix[j][i];
            matrix[j][i] = matrix[i][j] - matrix[j][i];
            matrix[i][j] = matrix[i][j] - matrix[j][i];
        }
    }
}

pub fn rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>) {
    let rows = matrix.len();
    let cols = matrix[0].len();

    // 补全非方阵
    if rows > cols {
        matrix.iter_mut().for_each(|row| {
            row.resize(rows, 0);
        })
    } else if cols > rows {
        matrix.resize(cols, vec![0; cols]);
    }

    matrix.reverse();
    transpose(matrix);

    // 恢复到原本的行列（注意行列经过上面的操作后被交换了）
    if rows > cols {
        matrix.resize(cols, vec![0; cols]);
    } else if cols > rows {
        matrix.iter_mut().for_each(|row| {
            row.resize(rows, 0);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix_1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3],]);
    }

    #[test]
    fn test_rotate_matrix_2() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![3, 1], vec![4, 2],]);
    }

    #[test]
    fn test_rotate_matrix_3() {
        let mut matrix = vec![vec![1]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![1],]);
    }

    #[test]
    fn test_rotate_matrix_4() {
        let mut matrix = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![5, 3, 1], vec![6, 4, 2],]);
    }
}
