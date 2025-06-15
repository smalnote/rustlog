pub struct Solution;

// 给定一个数，可以将其中的数字全部替换成另一个数字，注意替换后的数不能为0；
// 求替换后得到的最大的数和最小的数之间的差值；
//
// 要使数最大或最小，需要优先考虑对高位进行替换；
// 替换后最大，将非 9 的最高位的数字替换成 9;
// 替换后最小，若最高为非 1，则替换成 1，若最高位为1，则顺序找非0且非最高位的替换成0;
//
impl Solution {
    pub fn max_diff(num: i32) -> i32 {
        let mut digits: Vec<u8> = Vec::new();
        let mut num = num;
        while num > 0 {
            digits.push((num % 10) as u8);
            num /= 10;
        }
        let mut be_digits = digits.clone();
        be_digits.reverse();

        let mut r = 0_u8;
        for digit in be_digits.iter() {
            if *digit < 9 {
                r = *digit;
                break;
            }
        }
        let mut max = 0;
        let mut base = 1;
        for digit in digits.iter() {
            if *digit == r {
                max += 9 * base;
            } else {
                max += *digit as i32 * base;
            }
            base *= 10;
        }

        let min = if be_digits[0] > 1 {
            let mut min = 0;
            base = 1;
            for digit in digits.iter() {
                if *digit == be_digits[0] {
                    min += base;
                } else {
                    min += base * *digit as i32;
                }
                base *= 10;
            }
            min
        } else {
            let mut r = 0;
            for digit in be_digits.iter() {
                if *digit == 0 || *digit == be_digits[0] {
                    continue;
                }
                if *digit != 0 {
                    r = *digit;
                    break;
                }
            }
            let mut min = 0;
            base = 1;
            for digit in digits.iter() {
                if *digit != r {
                    min += *digit as i32 * base;
                }
                base *= 10;
            }
            min
        };

        max - min
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_555() {
        assert_eq!(Solution::max_diff(555), 888);
        assert_eq!(Solution::max_diff(9), 8);
        assert_eq!(Solution::max_diff(90), 89);
        assert_eq!(Solution::max_diff(123456), 820000);
        assert_eq!(Solution::max_diff(111), 888);
    }
}
