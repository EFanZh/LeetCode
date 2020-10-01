pub struct Solution;

impl Solution {
    pub fn is_ugly(mut num: i32) -> bool {
        if num <= 0 {
            false
        } else {
            while num % 2 == 0 {
                num /= 2;
            }

            while num % 3 == 0 {
                num /= 3;
            }

            while num % 5 == 0 {
                num /= 5;
            }

            num == 1
        }
    }
}

impl super::Solution for Solution {
    fn is_ugly(num: i32) -> bool {
        Self::is_ugly(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
