pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
        let mut positions = [(0, 0); 26];
        let mut count = 0;
        let mut result = 0;

        for (i, ch) in (1..).zip(s.bytes()) {
            let (prev_2, prev_1) = &mut positions[ch as usize - 'A' as usize];

            count += i + *prev_2 - *prev_1 * 2;
            result += count;

            *prev_2 = *prev_1;
            *prev_1 = i;
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
