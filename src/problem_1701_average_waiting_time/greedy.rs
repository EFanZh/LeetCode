pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut total_waiting_time = 0;
        let mut free_time = 0;
        let n = customers.len();

        for customer in customers {
            let [arrival, time]: [_; 2] = customer.try_into().ok().unwrap();

            free_time = free_time.max(arrival) + time;
            total_waiting_time += i64::from(free_time - arrival);
        }

        #[allow(clippy::cast_precision_loss)] // Expected.
        let result = (total_waiting_time as f64) / (n as f64);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        Self::average_waiting_time(customers)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
