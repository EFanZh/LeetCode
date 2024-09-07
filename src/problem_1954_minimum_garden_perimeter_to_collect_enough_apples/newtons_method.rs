pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_perimeter(num: i64) -> i64 {
        let num = num as u64;

        #[expect(clippy::cast_precision_loss, reason = "optimal")]
        let mut x = ((num / 4) as f64).cbrt() as u64;

        loop {
            x = (x * x * (x * 8 + 6) + num) / (x * (x * 12 + 12) + 2);

            let count = x * (x * (x * 4 + 6) + 2);

            if count <= num {
                if count < num {
                    x += 1;
                }

                break;
            }
        }

        (x * 8) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_perimeter(num: i64) -> i64 {
        Self::minimum_perimeter(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
