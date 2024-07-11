pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn split_painting(segments: Vec<Vec<i32>>) -> Vec<Vec<i64>> {
        let n = segments.len();
        let mut events = Vec::with_capacity(n * 2);

        for segment in segments {
            let [left, right, color]: [_; 3] = segment.try_into().ok().unwrap();

            events.extend([(left as u32, color), (right as u32, -color)]);
        }

        events.sort_unstable_by_key(|&(x, _)| x);

        let mut result = Vec::with_capacity(n);
        let mut prev_x = 0;
        let mut prev_sum = 0;

        for (x, color) in events {
            if x != prev_x {
                if prev_sum != 0 {
                    result.push(vec![i64::from(prev_x), i64::from(x), prev_sum]);
                }

                prev_x = x;
            }

            prev_sum += i64::from(color);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn split_painting(segments: Vec<Vec<i32>>) -> Vec<Vec<i64>> {
        Self::split_painting(segments)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
