pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn distance_traveled(main_tank: i32, additional_tank: i32) -> i32 {
        let (main_tank, additional_tank) = (main_tank as u32, additional_tank as u32);

        (10 * (main_tank + u32::min((main_tank - 1) / 4, additional_tank))) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn distance_traveled(main_tank: i32, additional_tank: i32) -> i32 {
        Self::distance_traveled(main_tank, additional_tank)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
