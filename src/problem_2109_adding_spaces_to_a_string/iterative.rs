pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn add_spaces(s: String, indices: Vec<i32>) -> String {
        let mut result = String::with_capacity(s.len() + indices.len());
        let mut prev_end = 0;

        for i in indices {
            let i = i as u32 as usize;

            result.push_str(&s[prev_end..i]);
            result.push(' ');
            prev_end = i;
        }

        result.push_str(&s[prev_end..s.len()]);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn add_spaces(s: String, indices: Vec<i32>) -> String {
        Self::add_spaces(s, indices)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
