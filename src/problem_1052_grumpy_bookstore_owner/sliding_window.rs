pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let make_iter = || {
            customers
                .iter()
                .zip(&grumpy)
                .map(|(customer, is_grumpy)| customer * is_grumpy)
        };

        let mut extra = 0;
        let mut iter_right = make_iter();

        for weight in iter_right.by_ref().take(minutes as _) {
            extra += weight;
        }

        let mut max_extra = extra;

        for (old, new) in make_iter().zip(iter_right) {
            extra = extra - old + new;
            max_extra = max_extra.max(extra);
        }

        let mut result = max_extra;

        for (customer, is_grumpy) in customers.iter().zip(&grumpy) {
            result += customer * (1 - is_grumpy);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        Self::max_satisfied(customers, grumpy, minutes)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
