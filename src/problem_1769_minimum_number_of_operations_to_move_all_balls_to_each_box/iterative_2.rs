pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let mut right_balls = 0;
        let mut right_cost = 0;

        for (i, c) in (0..).zip(boxes.bytes()) {
            let c = i32::from(c - b'0');

            right_balls += c;
            right_cost += i & -c;
        }

        let mut left_cost = 0;
        let mut left_balls = 0;

        boxes
            .bytes()
            .map(|c| {
                let c = i32::from(c - b'0');
                let result = left_cost + right_cost;

                left_balls += c;
                right_balls -= c;
                left_cost += left_balls;
                right_cost -= right_balls;

                result
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_operations(boxes: String) -> Vec<i32> {
        Self::min_operations(boxes)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
