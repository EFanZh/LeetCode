pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn elevator_requests(n: i32, requests: Vec<i32>) -> i32 {
        _ = n;

        let mut result = 0;
        let mut prev = 0;

        for request in requests {
            result += request.abs_diff(prev);
            prev = request;
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn elevator_requests(n: i32, requests: Vec<i32>) -> i32 {
        Self::elevator_requests(n, requests)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
