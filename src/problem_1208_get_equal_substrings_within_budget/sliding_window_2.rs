pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let max_cost = max_cost as u32;

        let mut iter = s.bytes().zip(t.bytes()).map(|(lhs, rhs)| u32::from(lhs.abs_diff(rhs)));

        let mut start = 0;
        let mut cost = 0;

        for value in iter.clone() {
            cost += value;

            if cost > max_cost {
                cost -= iter.next().unwrap();
                start += 1;
            }
        }

        s.len() as i32 - start
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
