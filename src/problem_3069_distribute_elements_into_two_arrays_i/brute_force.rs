pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let mut arr1 = (Vec::with_capacity(nums.len()), 0);
        let mut arr2 = (Vec::with_capacity(nums.len()), 0);
        let (left, right) = nums.split_at(2);

        [arr1.1, arr2.1] = left.try_into().ok().unwrap();

        arr1.0.push(arr1.1);
        arr2.0.push(arr2.1);

        for &num in right {
            let arr = if arr1.1 > arr2.1 { &mut arr1 } else { &mut arr2 };

            arr.0.push(num);
            arr.1 = num;
        }

        arr1.0.extend(arr2.0);

        arr1.0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn result_array(nums: Vec<i32>) -> Vec<i32> {
        Self::result_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
