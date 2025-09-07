pub struct Solution;

// 在一个按行从左到右,从上到下升序排列的矩阵中搜索目标数字.
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (row, column) = (matrix.len() as i32, matrix[0].len() as i32);
        let (mut lo, mut hi) = (0, row * column - 1);
        while lo <= hi {
            let mid = (lo + hi) / 2;
            let (r, c) = ((mid / column) as usize, (mid % column) as usize);
            if target < matrix[r][c] {
                hi = mid - 1;
            } else if target > matrix[r][c] {
                lo = mid + 1;
            } else {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_example() {
        assert!(Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            3
        ));
        assert!(!Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            0
        ));
        assert!(!Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            61
        ));
        assert!(!Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            12
        ));
    }
}
