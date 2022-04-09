pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32 {
        let mut tokens = tokens;
        let mut power = power as u32;

        tokens.sort_unstable();

        let mut iter = tokens.into_iter();
        let mut score = 0_u32;

        while let Some(left_value) = iter.next() {
            if let Some(next_power) = power.checked_sub(left_value as _) {
                power = next_power;
                score += 1;
            } else if score != 0 {
                if let Some(right_value) = iter.next_back() {
                    power += (right_value - left_value) as u32;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        score as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32 {
        Self::bag_of_tokens_score(tokens, power)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
