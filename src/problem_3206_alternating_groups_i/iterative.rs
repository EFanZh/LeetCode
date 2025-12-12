pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
        assert!(colors.len() > 2);

        let mut state = (colors[colors.len() - 2] << 1) | colors[colors.len() - 1];
        let mut result = 0;

        for color in colors {
            state = (state << 1) | color;
            result += i32::from(matches!(state & 0b_111, 0b_010 | 0b_101));
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
        Self::number_of_alternating_groups(colors)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
