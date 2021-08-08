pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        (left..=right)
            .filter(|&x| {
                let mut k = x;

                loop {
                    let digit = k % 10;

                    if digit == 0 || x % digit != 0 {
                        return false;
                    }

                    k /= 10;

                    if k == 0 {
                        return true;
                    }
                }
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        Self::self_dividing_numbers(left, right)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
