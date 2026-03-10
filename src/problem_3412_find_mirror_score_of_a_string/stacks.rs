pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn calculate_score(s: String) -> i64 {
        let mut stacks = [const { Vec::<u32>::new() }; 26];
        let mut result = 0;

        (0..).zip(s.bytes()).for_each(|(i, c)| {
            let c = usize::from(c);

            if let Some(j) = stacks[usize::from(25 + b'a') - c].pop() {
                result += u64::from(i - j);
            } else {
                stacks[c - usize::from(b'a')].push(i);
            }
        });

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn calculate_score(s: String) -> i64 {
        Self::calculate_score(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
