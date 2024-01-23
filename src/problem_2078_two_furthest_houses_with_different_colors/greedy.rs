pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let mut iter = colors.iter().copied();
        let first = iter.next().unwrap();
        let mut left_length = 1;

        while iter.next() == Some(first) {
            left_length += 1;
        }

        let mut right_length = 0;

        while iter.next_back() == Some(first) {
            right_length += 1;
        }

        (colors.len() - left_length.min(right_length)) as i32 - 1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_distance(colors: Vec<i32>) -> i32 {
        Self::max_distance(colors)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
