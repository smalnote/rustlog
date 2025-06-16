pub struct Solution;

/// 给定一个整数矩阵,找出其中最长的严格递增的路径,返回其长度,路径不可以越界回环.
///
/// 记 dp[i][j] 为以 matrix[i][j] 开始的最长路径;
/// dp[i][j] = max(dp[i-1][j], dp[i+1][j], dp[i][j-1], dp[i][j+1]) + 1, where
/// matrix[i-1][j] > matrix[i][j] etc
impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let mut lip = Lip::from(matrix);
        lip.find()
    }
}

struct Lip {
    matrix: Vec<Vec<i32>>,
    dp: Vec<Vec<Option<i32>>>,
    m: i32,
    n: i32,
}

impl Lip {
    fn from(matrix: Vec<Vec<i32>>) -> Self {
        let (m, n) = (matrix.len(), matrix[0].len());
        let dp = vec![vec![None; n]; m];
        Self {
            matrix,
            dp,
            m: m as i32,
            n: n as i32,
        }
    }

    fn find(&mut self) -> i32 {
        let mut max = 0;
        for i in 0..self.m as usize {
            for j in 0..self.n as usize {
                let tmp = self.start_from(i, j);
                if max < tmp {
                    max = tmp;
                }
            }
        }
        max
    }

    fn start_from(&mut self, i: usize, j: usize) -> i32 {
        if let Some(l) = self.dp[i][j] {
            return l;
        }
        let mut max = 0;
        for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (p, q) = (i as i32 + dir.0, j as i32 + dir.1);
            if p < 0 || p >= self.m || q < 0 || q >= self.n {
                continue;
            }
            let (p, q) = (p as usize, q as usize);
            if self.matrix[i][j] < self.matrix[p][q] {
                let tmp = self.start_from(p, q);
                if max < tmp {
                    max = tmp;
                }
            }
        }
        max += 1;
        self.dp[i][j] = Some(max);
        max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_example() {
        assert_eq!(
            Solution::longest_increasing_path(vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]]),
            4
        );
    }
}
