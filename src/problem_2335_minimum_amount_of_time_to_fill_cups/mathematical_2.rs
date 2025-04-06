pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn fill_cups(amount: Vec<i32>) -> i32 {
        let (max, total) = amount
            .into_iter()
            .fold((0, 0), |(max, total), x| (max.max(x as u32), (total + x as u32)));

        max.max(total.div_ceil(2)) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn fill_cups(amount: Vec<i32>) -> i32 {
        Self::fill_cups(amount)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
