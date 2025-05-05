pub struct Solution {}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        match (x, n) {
            (1.0, _) => 1.0,
            (-1.0, n) => {
                if n & 1 == 0 {
                    1.0
                } else {
                    -1.0
                }
            }
            (_, 0) => 1.0,
            (x, 1) => x,
            (x, ..=-1) => 1.0 / Self::my_pow(x, if n == i32::MIN { i32::MAX } else { -n }),
            (2.0, n) => (1 << if n < 32 { n } else { 31 }) as f64,
            (x, 1..) => {
                let half = Self::my_pow(x, n / 2);
                if n & 1 == 0 {
                    return half * half;
                }
                half * half * x
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_my_pow() {
        assert_eq!(Solution::my_pow(3.0, 3), 27.0);
        assert_eq!(Solution::my_pow(-1.0, i32::MIN), 1.0);
    }
}
