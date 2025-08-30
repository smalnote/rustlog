pub struct Solution;

// 自然数序列 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11
// 求这个序列中每 n 位数，如 nth(9) = 0, nth(10) = 1,
//
// 1 位数的有 9 个, 两位数的有 (99-10+1) 90 个，三位数的有 900 个
impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut base = 1;
        let mut c = 1;
        let mut n = n as i64;
        while n > base * 9 * c {
            n -= base * 9 * c;
            base *= 10;
            c += 1;
        }
        // 从 base 开始的第 n  位，且 base 开始的数每个有 c 位
        let (base_nth, d) = ((n - 1) / c, (n - 1) % c);
        let mut number = base + base_nth;
        for _ in 1..(c - d) {
            number /= 10;
        }
        (number % 10) as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_example() {
        assert_eq!(Solution::find_nth_digit(1000000000), 1);
        assert_eq!(Solution::find_nth_digit(9), 9);
        assert_eq!(Solution::find_nth_digit(11), 0);
        assert_eq!(Solution::find_nth_digit(100), 5);
    }
}
