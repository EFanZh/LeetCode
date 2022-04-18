pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let tops = tops.as_slice();
        let bottoms = bottoms.as_slice();
        let mut counts = [0_u16; 6];

        for nums in [tops, bottoms] {
            for &num in nums {
                counts[num as usize - 1] += 1;
            }
        }

        let n = tops.len();

        if let Some(expected) = (0_u8..)
            .zip(counts)
            .find_map(|(value, count)| (usize::from(count) >= n).then(|| i32::from(value) + 1))
        {
            let mut count_1 = 0;
            let mut count_2 = 0;

            for (&source, &target) in tops.iter().zip(bottoms) {
                match (source == expected, target == expected) {
                    (false, false) => return -1,
                    (false, true) => count_1 += 1,
                    (true, false) => count_2 += 1,
                    (true, true) => {}
                }
            }

            count_1.min(count_2)
        } else {
            -1
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        Self::min_domino_rotations(tops, bottoms)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
