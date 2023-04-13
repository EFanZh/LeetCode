pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut prev = i32::MAX;
        let mut i = n;

        loop {
            if let Some(&num) = arr.get(i.wrapping_sub(1)) {
                if num > prev {
                    break;
                }

                prev = num;
                i -= 1;
            } else {
                return 0;
            }
        }

        prev = i32::MIN;

        let mut length = i;

        'outer: while let Some(&right) = arr.get(i) {
            loop {
                let left = arr[i - length];

                if left < prev {
                    break 'outer;
                }

                prev = left;

                if right < left {
                    break;
                }

                length -= 1;
            }

            i += 1;
        }

        loop {
            let left = arr[i - length];

            if left < prev {
                break;
            }

            prev = left;
            length -= 1;
        }

        length as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        Self::find_length_of_shortest_subarray(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
