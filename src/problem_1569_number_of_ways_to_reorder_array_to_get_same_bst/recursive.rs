pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

const MODULUS: u32 = 1_000_000_007;

struct Context<'a> {
    combinations: &'a mut [u32],
    n: usize,
    buffer: &'a mut [i32],
}

impl Context<'_> {
    fn get_combination(&self, n: usize, k: usize) -> u32 {
        self.combinations[self.n * n + k]
    }
}

impl Solution {
    fn combinations(n: usize) -> Box<[u32]> {
        let mut buffer = vec![0_u32; n * n].into_boxed_slice();
        let mut iter = buffer.chunks_exact_mut(n);
        let mut top_row = iter.next().unwrap();

        top_row[0] = 1;

        for (count, row) in (2..).zip(iter) {
            let mut top_left = 0;

            for (target, &mut top) in row.iter_mut().zip(top_row).take(count) {
                *target = top_left + top;
                *target = (*target).checked_sub(MODULUS).unwrap_or(*target);

                top_left = top;
            }

            top_row = row;
        }

        buffer
    }

    fn partition<'a>(nums: &'a mut [i32], key: i32, buffer: &mut [i32]) -> (&'a mut [i32], &'a mut [i32]) {
        assert!(nums.len() <= buffer.len());

        let mut left_count = 0;
        let mut i = 0;

        while let Some(&num) = nums.get(i) {
            if num < key {
                nums[left_count] = num;
                left_count += 1;
            } else {
                buffer[i - left_count] = num;
            }

            i += 1;
        }

        let result = nums.split_at_mut(left_count);

        result.1.copy_from_slice(&buffer[..i - left_count]);

        result
    }

    fn multiply(lhs: u32, rhs: u32) -> u32 {
        ((u64::from(lhs) * u64::from(rhs)) % u64::from(MODULUS)) as _
    }

    fn helper(context: &mut Context, nums: &mut [i32]) -> u32 {
        if let Some((&mut first, rest)) = nums.split_first_mut() {
            let (left, right) = Self::partition(rest, first, context.buffer);
            let left_count = Self::helper(context, left);
            let right_count = Self::helper(context, right);
            let combinations = context.get_combination(left.len() + right.len(), left.len());

            Self::multiply(Self::multiply(left_count, right_count), combinations)
        } else {
            1
        }
    }

    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();

        let total_count = Self::helper(
            &mut Context {
                combinations: &mut Self::combinations(n),
                n,
                buffer: &mut vec![0; n - 1].into_boxed_slice(),
            },
            &mut nums,
        );

        let result = total_count + MODULUS - 1;

        result.checked_sub(MODULUS).unwrap_or(result) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_of_ways(nums: Vec<i32>) -> i32 {
        Self::num_of_ways(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
