pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let k = k.cast_unsigned();

        nums.sort_unstable_by_key(|num| num.cast_unsigned());

        nums.chunks_exact(3)
            .map(|window| {
                let window @ [min, _, max] = window.try_into().ok().unwrap();

                if (max - min).cast_unsigned() <= k {
                    Ok(window.to_vec())
                } else {
                    Err(())
                }
            })
            .collect::<Result<_, _>>()
            .unwrap_or_default()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        Self::divide_array(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
