pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_balanced_shipments(weight: Vec<i32>) -> i32 {
        let mut iter = weight.iter().copied().map(i32::cast_unsigned);
        let mut result = 0;

        'outer: while let Some(mut prev) = iter.next() {
            loop {
                if let Some(weight) = iter.next() {
                    if weight < prev {
                        result += 1;

                        break;
                    }

                    prev = weight;
                } else {
                    break 'outer;
                }
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
