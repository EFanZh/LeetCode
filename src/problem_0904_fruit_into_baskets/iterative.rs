pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let (&first, rest) = fruits.split_first().unwrap();

        let result = if let Some((i, &second)) = (2..).zip(rest).find(|&(_, &x)| x != first) {
            let mut prev_1_fruit = first;
            let mut prev_2_fruit = second;
            let mut length = i;
            let mut right_chunk_length = 1;
            let mut result = length;

            for &fruit in &fruits[length..] {
                if fruit == prev_2_fruit {
                    right_chunk_length += 1;
                } else {
                    if fruit != prev_1_fruit {
                        length = right_chunk_length;
                    }

                    prev_1_fruit = prev_2_fruit;
                    prev_2_fruit = fruit;
                    right_chunk_length = 1;
                }

                length += 1;
                result = result.max(length);
            }

            result
        } else {
            fruits.len()
        };

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn total_fruit(fruits: Vec<i32>) -> i32 {
        Self::total_fruit(fruits)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
