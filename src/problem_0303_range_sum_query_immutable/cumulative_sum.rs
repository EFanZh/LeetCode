struct NumArray {
    sums: Box<[i32]>,
}

impl NumArray {
    fn new(mut nums: Vec<i32>) -> Self {
        let mut sum = 0;

        for num in &mut nums {
            sum += *num;

            *num = sum;
        }

        Self {
            sums: nums.into_boxed_slice(),
        }
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        self.sums[j as usize] - self.sums.get((i as usize).wrapping_sub(1)).copied().unwrap_or(0)
    }
}

impl super::NumArray for NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self::new(nums)
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        self.sum_range(i, j)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::NumArray>();
    }
}
