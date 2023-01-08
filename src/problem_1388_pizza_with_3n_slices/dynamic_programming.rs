pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper<'a>(
        slices: &[i32],
        mut cache_1: &'a mut [u32],
        mut cache_2: &'a mut [u32],
        mut temp: &'a mut [u32],
    ) -> u32 {
        let (&first, rest) = slices.split_first().unwrap();

        cache_2[1] = first as _;

        for &num in rest {
            let num = num as u32;

            for (target, (&prev_1, &prev_2)) in temp[1..].iter_mut().zip(cache_1.iter().zip(&cache_2[1..])) {
                *target = (prev_1 + num).max(prev_2);
            }

            let new_temp = cache_1;

            cache_1 = cache_2;
            cache_2 = temp;
            temp = new_temp;
        }

        cache_2[(slices.len() + 1) / 3]
    }

    pub fn max_size_slices(slices: Vec<i32>) -> i32 {
        let n = slices.len();
        let total_deletions = n / 3;
        let mut buffer = vec![0; n + 3];
        let (cache_1, rest) = buffer.split_at_mut(total_deletions + 1);
        let (cache_2, temp) = rest.split_at_mut(total_deletions + 1);
        let candidate_1 = Self::helper(&slices[1..], cache_1, cache_2, temp);

        cache_1.fill(0);
        cache_2.fill(0);
        temp.fill(0);

        let candidate_2 = Self::helper(&slices[..n - 1], cache_1, cache_2, temp);

        candidate_1.max(candidate_2) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_size_slices(slices: Vec<i32>) -> i32 {
        Self::max_size_slices(slices)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
