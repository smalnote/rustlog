pub struct Solution;

/// 给定一个数 n,返回一个数组,按顺序代表 [0,n] 中 1 的个数;
/// 0 -> 0
/// 1 -> 1
/// 2 -> 10
/// 3 -> 11
/// 4 -> 100
/// 5 -> 101
/// 6 -> 110
/// 7 -> 111
/// 8 -> 1000
/// 可以将数字按 bit 位数分组
/// bits[0] = 0
/// bits[1] = 1
///
/// bits[2] = 1+bits[0]
/// bits[3] = 1+bits[1]
///
/// bits[4] = 1+bits[0]
/// bits[5] = 1+bits[1]
/// bits[6] = 1+bits[2]
/// bits[7] = 1+bits[3]
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut bits = Vec::with_capacity(n as usize + 1);
        bits.push(0);
        let mut num = 1;
        let mut count = 1;
        while num <= n {
            let mut base = 0;
            while num < 1 << count && num <= n {
                bits.push(1 + bits[base]);
                base += 1;
                num += 1;
            }
            count += 1;
        }
        bits
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_example() {
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
