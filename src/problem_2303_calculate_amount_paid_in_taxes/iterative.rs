pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
        let income = income as u32;
        let mut prev = 0;
        let mut taxes = 0;

        for bracket in brackets {
            let [upper, percent] = <[_; 2]>::map(bracket.try_into().ok().unwrap(), |x| x as u32);

            if income < upper {
                taxes += (income - prev) * percent;

                break;
            }

            taxes += (upper - prev) * percent;
            prev = upper;
        }

        f64::from(taxes) / 100.0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
        Self::calculate_tax(brackets, income)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
