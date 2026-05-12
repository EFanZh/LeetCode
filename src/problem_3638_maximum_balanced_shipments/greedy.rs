pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_balanced_shipments(weight: Vec<i32>) -> i32 {
        let weight = weight.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let mut prev = 0;
        let mut result = 0;

        for weight in weight {
            if weight < prev {
                prev = 0;
                result += 1;
            } else {
                prev = weight;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_balanced_shipments(weight: Vec<i32>) -> i32 {
        Self::max_balanced_shipments(weight)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
