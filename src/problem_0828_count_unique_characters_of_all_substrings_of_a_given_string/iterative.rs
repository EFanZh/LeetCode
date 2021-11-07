pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
        let mut result = 0;
        let mut count_and_positions = [(0, 0); 26];
        let mut i = 1;

        for c in s.bytes() {
            let (left_count, position) = &mut count_and_positions[usize::from(c) - usize::from(b'A')];
            let right_count = i - *position;

            result += *left_count * right_count;

            *left_count = right_count;
            *position = i;
            i += 1;
        }

        for &(left_count, position) in &count_and_positions {
            result += left_count * (i - position);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn unique_letter_string(s: String) -> i32 {
        Self::unique_letter_string(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
