pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let n = n as u32;

        ((n + 1) * (n + 2) * (n + 3) * (n + 4) / 24) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_vowel_strings(n: i32) -> i32 {
        Self::count_vowel_strings(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
