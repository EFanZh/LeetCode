pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let iter = differences.iter();
        let mut iter_2 = iter.clone();

        iter_2.next();

        let mut current = 0;
        let mut min = 0;
        let mut max = 0;
        let mut iter = differences.chunks_exact(2);

        for diffs in iter.by_ref() {
            let &[diff_1, diff_2]: &[_; 2] = diffs.try_into().ok().unwrap();

            current += diff_1;

            let left = current;

            current += diff_2;

            if current < left {
                min = min.min(current);
                max = max.max(left);
            } else {
                min = min.min(left);
                max = max.max(current);
            }
        }

        if let &[diff] = iter.remainder() {
            current += diff;

            if current < min {
                min = current;
            } else if current > max {
                max = current;
            }
        }

        let actual_range = max - min;
        let available_range = upper - lower;

        (available_range as u32 + 1).saturating_sub(actual_range as u32) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        Self::number_of_arrays(differences, lower, upper)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
