pub mod iterative;

pub trait Solution {
    fn three_equal_parts(arr: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn trim_leading_zeros(values: &[i32]) -> &[i32] {
        let split = values.iter().take_while(|&&x| x == 0).count();

        &values[split..]
    }

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 0, 1, 0, 1] as &[_], true),
            (&[1, 1, 0, 1, 1], false),
            (&[1, 1, 0, 0, 1], true),
            (&[1, 0, 1, 1, 0], false),
            (&[1, 1, 1, 1, 1, 1, 0, 1, 1, 1], true),
            (&[1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1], false),
            (&[0, 0, 0, 0, 0], true),
        ];

        for (arr, expected) in test_cases {
            let result = S::three_equal_parts(arr.to_vec());

            if expected {
                let [i, j]: [_; 2] = result.try_into().unwrap();
                let (i, j) = (i as usize, j as usize);
                let part_1 = trim_leading_zeros(&arr[..=i]);
                let part_2 = trim_leading_zeros(&arr[i + 1..j]);
                let part_3 = trim_leading_zeros(&arr[j..]);

                assert_eq!(part_1, part_2);
                assert_eq!(part_1, part_3);
            } else {
                assert_eq!(result, [-1, -1]);
            }
        }
    }
}
