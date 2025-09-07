pub struct Solution;

// 给定一个升序排列的数组,代表一个研究者的论文引用数 citations[i]
// 定义研究者的 h 指数为其至少有 h 篇论文引用数至少为 h, 取最大的 h
//
// 进行二分查找, h 0 ~ citations.len()
// 取 mid, >= citations[mid] 的论文数为 c = citations.len() - mid
// 若 citations[mid] >= c, 则有效 mid 候选, hi = mid
// 否则 mid 无效, lo = mid+1
// 保持 lo < hi, 结果为 lo
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0, citations.len());
        while lo < hi {
            let mid = (lo + hi) / 2;
            if citations[mid] as usize >= citations.len() - mid {
                hi = mid
            } else {
                lo = mid + 1
            }
        }
        (citations.len() - lo) as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_example() {
        assert_eq!(Solution::h_index(vec![0, 0, 0, 0, 0]), 0);
    }
}
