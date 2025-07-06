pub struct Solution;

// 一个字符串为 k-special 时,有对于字符串中的字符有 | freq[word[i]] - freq[word[j]] | <= k
// 即任意两个字符的频率的差值小于等于 k, 给定一个字符串,求最少删除多少个字符使其可以变成
// k-special的
// 将字符频率按从小到大排序有 f0 <= f1 <= f2 <= ... fm, 共 m 个不同的字符
// 选定一个频率 x, 保持其不变作为最终结果集中频率最小的,要达到 k-special
// 需要删除 < x 的, 对于 y > x + k 需要删除 y - x - k
// 只有26个小写字母,直接遍历即可, 复杂度 26^2
impl Solution {
    pub fn minimum_deletions(word: String, k: i32) -> i32 {
        let mut freqs = [0; 26];
        for c in word.chars() {
            freqs[c as usize - 'a' as usize] += 1;
        }
        freqs.sort();
        let mut min = word.len() as i32;
        for &x in freqs.iter() {
            let mut curr = 0;
            for &y in freqs.iter() {
                if y < x {
                    curr += y
                } else if y > x + k {
                    curr += y - x - k
                }
            }
            min = min.min(curr)
        }
        min
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_example() {
        assert_eq!(Solution::minimum_deletions("ykekkdhehmhhympxhgjyjsmmkxerplpeegaqwqmswpmkldlllrywjqyeqlmwyphgprsdorlllpmmwdwxsxgkwaogxgglokjykrqyhaasjjxalxwdkjexdqksayxqplwmmleevdkdqdvgelmdhkqgryrqawxeammjhpwqgvdhygyvyqahvkjrrjvgpgqxyywwdvpgelvsprqodrvewqyajwjsrmqgqmardoqjmpymmvxxqoqvhywderllksxapamejdslhwpohmeryemphplqlkddyhqgpqykdhrehxwsjvaqymykjodvodjgqahrejxlykmmaxywdgaoqvgegdggykqjwyagdohjwpdypdwlrjksqkjwrkekvxjllwkgxxmhrwmxswmyrmwldqosavkpksjxwjlldhyhhrrlrwarqkyogamxmpqyhsldhajagslmeehakrxjxpjjmjpydgkehesoygvosrhvyhrqmdhlomgmrqjrmxyvmapmspmdygkhsprqsaxsvsrkovdjprjjyworgqoakrwarjsryydpmvhvyalawsmlsdgolsxgaqhryemvkpkhqvvagmxoapmsmwkrakldlhyojqhjjghjxgksroqpoxqsorrelhqeseegpqpewxydvkvaoaldmsdpmvogaykhpxkjkwmslqjsdqowkqawxadevkswdhywrxkpvqxmgeolayqojqqwxoomyasjrqrjmoearskssppmxpgwrmsjlsrjyqrjkgwjwglxogmkqjpjkwyaqxymelsyxypqxrjvpmssoakksemjhvaxm".into(), 2), 2);
        assert_eq!(Solution::minimum_deletions("aabcaba".into(), 0), 3);
        assert_eq!(Solution::minimum_deletions("dabdcbdcdcd".into(), 2), 2);
    }
}
