pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut result = vec![0; n as _];
        let mut stack = Vec::new();

        for log in logs {
            let mut iter = log.split(':');
            let id = iter.next().unwrap().parse::<usize>().unwrap();
            let is_start = iter.next().unwrap().starts_with('s');
            let mut timestamp = iter.next().unwrap().parse::<i32>().unwrap();

            if is_start {
                if let Some(&(top_id, top_timestamp)) = stack.last() {
                    result[top_id] += timestamp - top_timestamp;
                }

                stack.push((id, timestamp));
            } else {
                let top_timestamp = stack.pop().unwrap().1;

                timestamp += 1;
                result[id] += timestamp - top_timestamp;

                if let Some((_, top_timestamp)) = stack.last_mut() {
                    *top_timestamp = timestamp;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        Self::exclusive_time(n, logs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
