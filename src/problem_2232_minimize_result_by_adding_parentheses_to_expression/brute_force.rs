pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimize_result(expression: String) -> String {
        let expression = expression.into_bytes();
        let plus_position = expression.iter().position(|&c| c == b'+').unwrap();
        let left = &expression[..plus_position];
        let right = &expression[plus_position + 1..];
        let mut left_pairs = [(0, 0); 8];
        let mut right_pairs = [(0, 0); 8];

        assert!(matches!((left.len(), right.len()), (1..=8, 1..=8)));

        // Fill part 1.

        let mut number = 0;

        left_pairs[0].0 = 1;

        for ((target, _), &digit) in left_pairs[1..].iter_mut().zip(left) {
            number = number * 10 + u32::from(digit - b'0');
            *target = number;
        }

        // Fill part 2.

        let mut number = 0;
        let mut base = 1;

        for ((_, target), &digit) in left_pairs.iter_mut().zip(left).rev() {
            number += u32::from(digit - b'0') * base;
            *target = number;
            base *= 10;
        }

        // Fill part 3.

        let mut number = 0;

        for ((target, _), &digit) in right_pairs.iter_mut().zip(right) {
            number = number * 10 + u32::from(digit - b'0');
            *target = number;
        }

        // Fill part 4.

        let mut number = 0;
        let mut base = 1;

        for ((_, target), &digit) in right_pairs.iter_mut().zip(&right[1..]).rev() {
            number += u32::from(digit - b'0') * base;
            *target = number;
            base *= 10;
        }

        right_pairs[right.len() - 1].1 = 1;

        // Check all combinations.

        let mut min = u32::MAX;
        let mut split = (0, 0);

        for (i, &(part_1, part_2)) in (0..).zip(&left_pairs[..left.len()]) {
            for (j, &(part_3, part_4)) in (1..).zip(&right_pairs[..right.len()]) {
                let value = part_1 * (part_2 + part_3) * part_4;

                if value < min {
                    min = value;
                    split = (i, j);
                }
            }
        }

        let (part_1, part_2) = left.split_at(split.0);
        let (part_3, part_4) = right.split_at(split.1);
        let mut result = Vec::with_capacity(expression.len() + 2);

        result.extend(part_1);
        result.push(b'(');
        result.extend(part_2);
        result.push(b'+');
        result.extend(part_3);
        result.push(b')');
        result.extend(part_4);

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimize_result(expression: String) -> String {
        Self::minimize_result(expression)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
