pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut iter = salary.iter().copied();
        let mut min = iter.next().unwrap();
        let mut max = min;
        let mut sum = min;

        for value in iter {
            sum += value;

            if value < min {
                min = value;
            } else if value > max {
                max = value;
            }
        }

        #[expect(clippy::cast_precision_loss, reason = "optimal")]
        let result = f64::from(sum - min - max) / ((salary.len() - 2) as f64);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn average(salary: Vec<i32>) -> f64 {
        Self::average(salary)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
