pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn get_min_swaps(num: String, k: i32) -> i32 {
        let mut num = num.into_bytes();
        let mut target_num = num.clone();
        let k = k as u32;
        let mut result = 0;

        for _ in 0..k {
            let mut i = target_num.len();
            let mut prev = 0;

            for &x in target_num.iter().rev() {
                i -= 1;

                let found = x < prev;

                prev = x;

                if found {
                    break;
                }
            }

            let j = i + 1 + target_num[i + 2..].partition_point(|&x| x > prev);

            target_num.swap(i, j);
            target_num[i + 1..].reverse();
        }

        for (i, &target) in target_num.iter().enumerate() {
            let j = num[i..].iter().position(|&c| c == target).unwrap();

            result += j;
            num[i..i + 1 + j].rotate_right(1);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_min_swaps(num: String, k: i32) -> i32 {
        Self::get_min_swaps(num, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
