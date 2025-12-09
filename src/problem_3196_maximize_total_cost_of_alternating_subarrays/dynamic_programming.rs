pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_total_cost(nums: Vec<i32>) -> i64 {
        let mut even_cost = 0;
        let mut odd_cost = i64::MIN / 2;

        for num in nums {
            let num = i64::from(num);
            let new_odd_cost = even_cost.max(odd_cost) + num;
            let new_even_cost = new_odd_cost.max(odd_cost - num);

            (even_cost, odd_cost) = (new_even_cost, new_odd_cost);
        }

        even_cost.max(odd_cost)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_total_cost(nums: Vec<i32>) -> i64 {
        Self::maximum_total_cost(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
