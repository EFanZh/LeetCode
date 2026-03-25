pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_weight(pizzas: Vec<i32>) -> i64 {
        let mut pizzas = pizzas.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let n = pizzas.len();
        let days = n / 4;
        let even_days = days / 2;
        let odd_days = days - even_days;
        let size_1 = even_days * 2;
        let size_2 = odd_days;
        let split = n - size_1 - size_2;

        pizzas.select_nth_unstable(split);

        let right = &mut pizzas[split..];

        right.select_nth_unstable(size_1);

        let (right_left, right_right) = right.split_at_mut(size_1);

        right_left.sort_unstable();

        right_left
            .iter()
            .step_by(2)
            .chain(&*right_right)
            .fold(0, |sum, &pizza| sum + i64::from(pizza))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_weight(pizzas: Vec<i32>) -> i64 {
        Self::max_weight(pizzas)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
