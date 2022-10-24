pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let max_cost = max_cost as u32;

        let mut iter = s
            .bytes()
            .zip(t.bytes())
            .map(|(lhs, rhs)| u32::from(if rhs < lhs { lhs - rhs } else { rhs - lhs }));

        let mut start = 0;
        let mut cost = 0;
        let mut result = 0_u32;

        for (i, value) in (1..).zip(iter.clone()) {
            cost += value;

            while cost > max_cost {
                cost -= iter.next().unwrap();
                start += 1;
            }

            result = result.max(i - start);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        Self::equal_substring(s, t, max_cost)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
