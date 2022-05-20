pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn prev_perm_opt1(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        let mut i = arr.len() - 1;
        let mut prev = arr[i];

        loop {
            i = i.wrapping_sub(1);

            if let Some(&num) = arr.get(i) {
                if prev < num {
                    let swap_with_candidates = &arr[i + 1..];
                    let j = swap_with_candidates.partition_point(|&x| x < num);
                    let swap_with_value = swap_with_candidates[j - 1];
                    let k = swap_with_candidates[..j - 1].partition_point(|&x| x < swap_with_value);

                    arr.swap(i, i + 1 + k);

                    break;
                }

                prev = num;
            } else {
                break;
            }
        }

        arr
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn prev_perm_opt1(arr: Vec<i32>) -> Vec<i32> {
        Self::prev_perm_opt1(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
