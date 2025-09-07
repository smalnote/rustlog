pub struct Solution;

/// 在一个每一行从左到右递增,每一列从上到下递增的矩阵中查找 target
///
/// 先查找 target 在哪一行,再查找其在哪一列
/// 如果 target 在某一行中,则 row[0] <= target, row[i] >= target 则 row[>=i] 全部不可能
/// 类似的,如果 row.last() < target, 则 row 及往上的行都不可能,
/// 圈定行的范围后,逐个二分查找
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (row, column) = (matrix.len(), matrix[0].len());
        let (mut lo, mut hi) = (0, row);

        while lo < hi {
            let mid = (lo + hi) / 2;
            if matrix[mid][0] > target {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        let row_hi = lo;
        (lo, hi) = (0, row_hi);
        while lo < hi {
            let mid = (lo + hi) / 2;
            if matrix[mid][column - 1] < target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        let row_lo = lo;
        for row in &matrix[row_lo..row_hi] {
            if row.binary_search(&target).is_ok() {
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
        assert!(!Solution::search_matrix(
            vec![
                vec![1, 4, 7, 11, 15],
                vec![2, 5, 8, 12, 19],
                vec![3, 6, 9, 16, 22],
                vec![10, 13, 14, 17, 24],
                vec![18, 21, 23, 26, 30]
            ],
            20
        ));
    }
}
