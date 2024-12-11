pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_groups(grades: Vec<i32>) -> i32 {
        let n = grades.len();
        let n_times_2 = n * 2;
        let mut guess = n;

        loop {
            let new_guess = (guess * guess + n_times_2) / (guess * 2 + 1);

            if new_guess < guess {
                guess = new_guess;
            } else {
                return guess as _;
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_groups(grades: Vec<i32>) -> i32 {
        Self::maximum_groups(grades)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
