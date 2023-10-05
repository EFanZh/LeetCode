pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_ways(s: String) -> i32 {
        let ones = s.bytes().filter(|&c| c == b'1').count();

        if ones % 3 == 0 {
            (if ones == 0 {
                let n = s.len() as u64;

                (n - 1) * (n - 2) / 2
            } else {
                let substring_ones = ones / 3;

                #[allow(clippy::unnecessary_lazy_evaluations)]
                let mut iter = (0..).zip(s.bytes()).filter(|&(_, c)| c == b'1').map(|(i, _)| i);

                let start_1 = iter.nth(substring_ones - 1).unwrap();
                let end_1 = iter.next().unwrap();
                let length_1 = end_1 - start_1;

                let start_2 = if substring_ones == 1 {
                    end_1
                } else {
                    iter.nth(substring_ones - 2).unwrap()
                };

                let end_2 = iter.next().unwrap();

                let length_2 = end_2 - start_2;

                length_1 * length_2
            } % 1_000_000_007) as _
        } else {
            0
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_ways(s: String) -> i32 {
        Self::num_ways(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
