pub struct Solution;

// 有一堆带有数值的石头，Alice 和 Bob 流从中取一个，Alice 先，如果当前已经取出的
// 石头数值总和是三的整数倍，则取出石头的人输出，如果最后没有剩下石头，则 Alice
// 输，两个人都按最优化方法玩，返回 Alice 是否能赢。
//
// 石的数值可以分为三种类型 x mod 3 = 0, 1, 2，取同一种类型的石头效果相同。
impl Solution {
    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        let mut mods = [0_usize; 3];
        stones.iter().for_each(|v| mods[(*v % 3) as usize] += 1);
        Solution::take(mods, 1, stones.len()) || Solution::take(mods, 2, stones.len())
    }

    fn take(mut mods: [usize; 3], m: usize, n: usize) -> bool {
        if mods[m] == 0 {
            return false;
        }
        mods[m] -= 1;
        let mut sum = m;
        for i in 1..n {
            // 需要证明在当前 sum，取任意不输的情况是等价的
            // 余 1, 可取 0, 1
            // 余 2，可取 0, 2
            if mods[0] > 0 && sum % 3 != 0 {
                mods[0] -= 1;
            } else if mods[1] > 0 && (sum + 1) % 3 != 0 {
                mods[1] -= 1;
                sum += 1;
            } else if mods[2] > 0 && (sum + 2) % 3 != 0 {
                mods[2] -= 1;
                sum += 2;
            } else {
                // 无法组成 mod 3 != 0，取奇数序号的是 Bob 输，Alice 赢
                return i & 1 == 1;
            }
        }
        // 取完了，Alice 输
        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_examples() {
        assert!(Solution::stone_game_ix(vec![2, 1]));
        assert! {!Solution::stone_game_ix(vec![2])};
        assert!(!Solution::stone_game_ix(vec![5, 1, 2, 4, 3]));
    }
}
