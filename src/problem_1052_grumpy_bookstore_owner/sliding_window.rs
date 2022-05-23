pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let make_iter = || customers.iter().zip(&grumpy);
        let mut base = 0;
        let mut extra = 0;
        let mut iter_right = make_iter();

        for (customer, is_grumpy) in iter_right.by_ref().take(minutes as _) {
            base += customer * (1 - is_grumpy);
            extra += customer * is_grumpy;
        }

        let mut max_extra = extra;

        for (old, new) in make_iter().zip(iter_right) {
            base += new.0 * (1 - new.1);
            extra = extra - old.0 * old.1 + new.0 * new.1;
            max_extra = max_extra.max(extra);
        }

        base + max_extra
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
