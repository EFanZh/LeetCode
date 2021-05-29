pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let length = s.len();

        for i in 0..length / 2 {
            s.swap(i, length - 1 - i);
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reverse_string(s: &mut Vec<char>) {
        Self::reverse_string(s);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
