pub struct Solution {}

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for x in digits.iter_mut().rev() {
            if *x == 9 {
                *x = 0;
            } else {
                *x += 1;

                return digits;
            }
        }

        digits.insert(0, 1);

        digits
    }
}

impl super::Solution for Solution {
    fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        Self::plus_one(digits)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
