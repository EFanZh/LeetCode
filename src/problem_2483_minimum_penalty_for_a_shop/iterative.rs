pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let mut penalty = customers.bytes().filter(|&c| c != b'N').count() as u32;
        let mut min_penalty = penalty;
        let mut min_penalty_time = 0;
        let mut time = 0;

        for c in customers.bytes() {
            time += 1;

            if c == b'N' {
                penalty += 1;
            } else {
                penalty -= 1;
            }

            if penalty < min_penalty {
                min_penalty = penalty;
                min_penalty_time = time;
            }
        }

        min_penalty_time
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn best_closing_time(customers: String) -> i32 {
        Self::best_closing_time(customers)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
