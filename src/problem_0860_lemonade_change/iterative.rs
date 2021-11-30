pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut five = 0_u32;
        let mut ten = 0_u32;

        for bill in bills {
            if bill == 5 {
                five += 1;
            } else {
                if five == 0 {
                    return false;
                }

                if bill == 10 {
                    five -= 1;
                    ten += 1;
                } else if ten == 0 {
                    if five < 3 {
                        return false;
                    }

                    five -= 3;
                } else {
                    five -= 1;
                    ten -= 1;
                }
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn lemonade_change(bills: Vec<i32>) -> bool {
        Self::lemonade_change(bills)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
