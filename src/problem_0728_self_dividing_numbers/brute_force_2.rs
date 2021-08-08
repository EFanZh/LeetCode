pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let (left, right) = (left as u32, right as u32);
        let mut result = Vec::with_capacity((right - left) as _);
        let mut x = left;

        loop {
            let mut k = x;

            loop {
                let digit = k % 10;

                if digit == 0 || x % digit != 0 {
                    break;
                }

                k /= 10;

                if k == 0 {
                    result.push(x as _);
                }
            }

            x += 1;

            if x > right {
                return result;
            }
        }
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
