pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32 {
        let mut candidate = arrival_time + delayed_time;

        if candidate >= 24 {
            candidate -= 24;
        }

        candidate
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32 {
        Self::find_delayed_arrival_time(arrival_time, delayed_time)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
