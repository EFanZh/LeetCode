pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    #[allow(clippy::ptr_arg)] // Expected.
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let n = arr.len();
        let mut required = 0;
        let mut i = 0;

        while let Some(&num) = arr.get(i) {
            required += if num == 0 { 2 } else { 1 };
            i += 1;

            if required >= n {
                break;
            }
        }

        if required > n {
            required -= 2;
            arr[required] = 0;
            i -= 1;
        }

        while required != 0 {
            i -= 1;

            let num = arr[i];

            required -= 1;
            arr[required] = num;

            if num == 0 {
                required -= 1;
                arr[required] = num;
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn duplicate_zeros(arr: &mut Vec<i32>) {
        Self::duplicate_zeros(arr);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
