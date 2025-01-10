pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn sort_as_u32(values: Vec<i32>) -> Vec<u32> {
        let mut values = values.into_iter().map(|x| x as u32).collect::<Vec<_>>();

        values.sort_unstable();

        values
    }

    pub fn min_wasted_space(packages: Vec<i32>, boxes: Vec<Vec<i32>>) -> i32 {
        let max_package = packages.iter().fold(0, |max, &size| max.max(size as _));
        let mut sums = vec![0_u64; max_package as usize + 1].into_boxed_slice();
        let mut required_space = 0;

        for &package in &packages {
            let package = package as u32;

            required_space += u64::from(package);
            sums[package as usize] += 1;
        }

        let mut sum = 0;

        for target in &mut *sums {
            sum += *target;
            *target = sum;
        }

        let mut result = u64::MAX;

        for boxes in boxes {
            let boxes = Self::sort_as_u32(boxes);

            if boxes.last().is_some_and(|&box_size| box_size >= max_package) {
                let mut used = 0;
                let mut prev_count = 0;

                for size in boxes {
                    if let Some(&current_count) = sums.get(size as usize) {
                        used += u64::from(size) * (current_count - prev_count);
                        prev_count = current_count;
                    } else {
                        used += u64::from(size) * (sum - prev_count);

                        break;
                    }
                }

                result = result.min(used);
            }
        }

        if result != u64::MAX {
            result -= required_space;
            result %= 1_000_000_007;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_wasted_space(packages: Vec<i32>, boxes: Vec<Vec<i32>>) -> i32 {
        Self::min_wasted_space(packages, boxes)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
