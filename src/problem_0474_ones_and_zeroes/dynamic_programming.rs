pub struct Solution;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let columns = n + 1;
        let mut cache = vec![0; columns * (m + 1)];

        for s in strs {
            let zeros = s.bytes().filter(|&c| c == b'0').count();
            let ones = s.len() - zeros;

            for max_zeros in (zeros..=m).rev() {
                for max_ones in (ones..=n).rev() {
                    cache[columns * max_zeros + max_ones] = cache[columns * max_zeros + max_ones]
                        .max(cache[columns * (max_zeros - zeros) + (max_ones - ones)] + 1);
                }
            }
        }

        *cache.last().unwrap()
    }
}

impl super::Solution for Solution {
    fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        Self::find_max_form(strs, m, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
