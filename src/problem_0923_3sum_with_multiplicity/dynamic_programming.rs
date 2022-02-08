pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn assign_add(lhs: &mut u32, rhs: u32) {
        *lhs += rhs;
        *lhs = lhs.checked_sub(1_000_000_007).unwrap_or(*lhs);
    }

    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        let target = target as u32 as usize;
        let mut cache = vec![(0, 0); target + 1];
        let mut result = 0;

        for num in arr {
            let num = num as u32 as usize;

            if let Some(&(_, two_sum_count)) = cache.get(target.wrapping_sub(num)) {
                Self::assign_add(&mut result, two_sum_count);

                for two_sum_target in num..=target {
                    let one_sum_count = cache[two_sum_target - num].0;

                    Self::assign_add(&mut cache[two_sum_target].1, one_sum_count);
                }

                cache[num].0 += 1;
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        Self::three_sum_multi(arr, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
