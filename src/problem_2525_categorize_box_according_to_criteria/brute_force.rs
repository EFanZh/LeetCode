pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        let (length, width, height) = (length as u32, width as u32, height as u32);

        let is_bulky = length >= 10000
            || width >= 10000
            || height >= 10000
            || length
                .checked_mul(width)
                .and_then(|x| x.checked_mul(height))
                .is_none_or(|volume| volume >= 1_000_000_000);

        let is_heavy = mass >= 100;

        String::from(match (is_bulky, is_heavy) {
            (false, false) => "Neither",
            (false, true) => "Heavy",
            (true, false) => "Bulky",
            (true, true) => "Both",
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        Self::categorize_box(length, width, height, mass)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
