pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    fn lower_bound(values: &[i32], target: i32) -> usize {
        values
            .binary_search_by(|&value| {
                if value < target {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
            .unwrap_err()
    }

    fn iter_chunks(values: &[i32], mut f: impl FnMut(&[i32], i32, usize)) {
        let mut prev = values[0];
        let mut start = 0;
        let mut i = 1;

        while let Some(&value) = values.get(i) {
            if value != prev {
                f(&values[..start], prev, i - start);

                prev = value;
                start = i;
            }

            i += 1;
        }

        f(&values[..start], prev, i - start);
    }

    pub fn num_friend_requests(ages: Vec<i32>) -> i32 {
        let ages = {
            let mut ages = ages;

            ages.sort_unstable();

            ages
        };

        let mut result = 0;
        let mut low = 0;
        let mut high = 0;

        Self::iter_chunks(&ages, |prev, age, count| {
            if age > 14 {
                result += count * (count - 1);
            }

            low += Self::lower_bound(&prev[low..], age / 2 + 8);
            high = high.max(low);
            high += Self::lower_bound(&prev[high..], age + 1);
            result += (high - low) * count;
        });

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_friend_requests(ages: Vec<i32>) -> i32 {
        Self::num_friend_requests(ages)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
