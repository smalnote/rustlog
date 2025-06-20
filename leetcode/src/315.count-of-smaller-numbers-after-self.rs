pub struct Solution;

// 给定一个数组,返回每个数后面比它小的数的数量
//
// 从左向右构造, 前序已经构造出来的有 (nums[i], counts[i], counts_equal[i])
// 其中 nums[i] 第 i 个数, counts[i] 为其右边小于 nums[i] 的个数, counts_equal[i]
// 右边与其相等个数+1
// 对于 nums[i-1], 找出前面构建的序列中, 小于等于 nums[i] 前下标最小的三元组
// (nums[k], counts[k], counts_equal[k], k)
// 可以确定 nums[i-1] 右边没有大于 nums[k] 且小于等于 nums[i-1] 的数(有的话那个作为新的 k)
// 则有
// 若 nums[i-1] > nums[k] => (nums[i-1], counts[k] + counts_equal[k] + counts(nums[i, k)), 1)
// 若 nums[i-1] = nums[k] => (nums[i-1], counts[k] + counts(nums[i, k)), counts_equal[k]+1)
// 若 k 不存在 => (nums[i-1], 0, 1)
// 为了方便搜索,把序列构造成有序(nums[k], counts[k], counts_equal[k]) 按 nums[i-1] 升序排列
// 如果 nums[i-1] 与 nums[k] 相等,原地更新
impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut tuples = Vec::<(i32, i32, i32, usize)>::with_capacity(nums.len());
        let mut results = vec![0; nums.len()];
        for (i, num) in nums.iter().rev().enumerate() {
            let i = nums.len() - 1 - i;
            results[i] = match tuples.binary_search_by(|(m, _, _, _)| m.cmp(num)) {
                Ok(at) => {
                    let mut prev = tuples[at];
                    for o in &nums[i + 1..prev.3] {
                        if o < num {
                            prev.1 += 1;
                        }
                    }
                    tuples[at] = (*num, prev.1, prev.2 + 1, i);
                    prev.1
                }
                Err(pos) => {
                    if pos > 0 {
                        let mut prev = tuples[pos - 1];
                        for o in &nums[i + 1..prev.3] {
                            if o < num {
                                prev.1 += 1;
                            }
                        }
                        tuples.insert(pos, (*num, prev.1 + prev.2, 1, i));
                        prev.1 + prev.2
                    } else {
                        tuples.insert(pos, (*num, 0, 1, i));
                        0
                    }
                }
            }
        }
        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_example() {
        assert_eq!(Solution::count_smaller(vec![2, 0, 1]), vec![2, 0, 0]);
        assert_eq!(
            Solution::count_smaller(vec![5, 2, 5, 4, 1]),
            vec![3, 1, 2, 1, 0]
        );
        assert_eq!(Solution::count_smaller(vec![5, 2, 6, 1]), vec![2, 1, 1, 0]);
        assert_eq!(Solution::count_smaller(vec![-1]), vec![0]);
        assert_eq!(Solution::count_smaller(vec![-1, -1]), vec![0, 0]);
    }
}
