pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        let tomato_slices = tomato_slices as u32;
        let cheese_slices = cheese_slices as u32;

        if tomato_slices.is_multiple_of(2)
            && let Some(jumbo_burgers) = (tomato_slices / 2).checked_sub(cheese_slices)
            && let Some(small_burgers) = cheese_slices.checked_sub(jumbo_burgers)
        {
            return vec![jumbo_burgers as _, small_burgers as _];
        }

        Vec::new()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        Self::num_of_burgers(tomato_slices, cheese_slices)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
