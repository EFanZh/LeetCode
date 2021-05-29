pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
        let mut result = 0;

        if let Some(&[first, second]) = a.get(..2) {
            let mut start = 1;
            let mut diff = second - first;
            let mut prev = second;
            let mut i = 2;

            while let Some(&value) = a.get(i) {
                let new_diff = value - prev;

                if new_diff != diff {
                    let length = i - start;

                    result += (length - 1) * length / 2;
                    start = i;
                    diff = new_diff;
                }

                prev = value;
                i += 1;
            }

            let length = i - start;

            result += (length - 1) * length / 2;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
        Self::number_of_arithmetic_slices(a)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
