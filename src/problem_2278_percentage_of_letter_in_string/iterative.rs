pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let letter = letter as u8;
        let count = s.bytes().filter(|&c| c == letter).count();

        (count * 100 / s.len()) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn percentage_letter(s: String, letter: char) -> i32 {
        Self::percentage_letter(s, letter)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
