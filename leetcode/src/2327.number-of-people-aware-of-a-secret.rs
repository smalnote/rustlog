pub struct Solution;

// 在 day 1 有一个人发现了一个密秘，
// 每个知道密秘的人在 +delay 后的每一天，会将这个密秘告诉一个新的人，直到 +forget 天停止（+forget
// 不告诉新的人），问在第 n 天，总共有多少个人知道密秘，结果对 10^9 + 7 取模。
impl Solution {
    const MOD: i32 = 1_000_000_000 + 7;
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        let mut knowns: Vec<i32> = vec![0; n as usize];
        knowns[0] = 1;
        let mut share_known = 0;
        let (n, delay, forget) = (n as usize, delay as usize, forget as usize);
        for i in delay..forget {
            share_known = (share_known + knowns[i - delay]) % Self::MOD;
            knowns[i] = share_known;
        }

        for i in forget..n {
            share_known = (share_known + knowns[i - delay]) % Self::MOD;
            share_known = (share_known - knowns[i - forget] + Self::MOD) % Self::MOD;
            knowns[i] = share_known;
        }

        knowns[n - forget..]
            .iter()
            .fold(0, |sum, know| (sum + *know) % Self::MOD)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::people_aware_of_secret(289, 7, 23), 790409951);
        assert_eq!(Solution::people_aware_of_secret(684, 18, 496), 653668527);
        assert_eq!(Solution::people_aware_of_secret(6, 2, 4), 5);
        assert_eq!(Solution::people_aware_of_secret(4, 1, 3), 6);
    }
}
