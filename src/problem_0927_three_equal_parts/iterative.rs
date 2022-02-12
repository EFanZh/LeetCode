pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper(total_length: u32, ones: &[u32]) -> [i32; 2] {
        if ones.len() % 3 == 0 {
            if let Some(&last_one) = ones.last() {
                let trailing_zeros = total_length - 1 - last_one;
                let expected_ones = ones.len() / 3;

                if ones[expected_ones] - ones[expected_ones - 1] > trailing_zeros
                    && ones[expected_ones * 2] - ones[expected_ones * 2 - 1] > trailing_zeros
                {
                    let mut iter = ones[..expected_ones]
                        .iter()
                        .copied()
                        .zip(ones[expected_ones..expected_ones * 2].iter().copied())
                        .zip(ones[expected_ones * 2..].iter().copied());

                    let ((mut prev_1, mut prev_2), mut prev_3) = iter.next().unwrap();

                    for ((i, j), k) in iter {
                        let diff = i - prev_1;

                        if (j - prev_2) != diff || (k - prev_3) != diff {
                            return [-1, -1];
                        }

                        prev_1 = i;
                        prev_2 = j;
                        prev_3 = k;
                    }

                    [
                        (ones[expected_ones - 1] + trailing_zeros) as _,
                        (ones[expected_ones * 2 - 1] + trailing_zeros + 1) as _,
                    ]
                } else {
                    [-1, -1]
                }
            } else {
                [0, 2]
            }
        } else {
            [-1, -1]
        }
    }

    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;

        let ones = (0..)
            .zip(&arr)
            .filter_map(|(i, &num)| if num == 0 { None } else { Some(i) })
            .collect::<Vec<_>>();

        let result = Self::helper(arr.len() as _, &ones);

        arr.splice(.., result.iter().copied());

        arr
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        Self::three_equal_parts(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
