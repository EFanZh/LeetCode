pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut count_a = 0_u32;
        let mut count_b = 0_u32;
        let mut count_l = 0_u32;
        let mut count_n = 0_u32;
        let mut count_o = 0_u32;

        for c in text.bytes() {
            match c {
                b'a' => count_a += 1,
                b'b' => count_b += 1,
                b'l' => count_l += 1,
                b'n' => count_n += 1,
                b'o' => count_o += 1,
                _ => {}
            }
        }

        count_a.min(count_b).min(count_l.min(count_o) / 2).min(count_n) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_number_of_balloons(text: String) -> i32 {
        Self::max_number_of_balloons(text)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
