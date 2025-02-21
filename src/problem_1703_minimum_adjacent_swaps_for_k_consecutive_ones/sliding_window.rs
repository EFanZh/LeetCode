pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn find_ones(nums: &mut Vec<i32>) {
        let mut ones = 0;
        let mut i = 0;

        while let Some(&num) = nums.get(i) {
            if num != 0 {
                nums[ones] = i as _;
                ones += 1;
            }

            i += 1;
        }

        nums.truncate(ones);
    }

    fn get_cost(window: &[i32]) -> u32 {
        let mut result = 0;
        let middle = window.len() / 2;
        let i32_as_u32 = |&x| x as u32;

        // Left cost.

        let mut iter = window[..=middle].iter().map(i32_as_u32);
        let mut prev = iter.next().unwrap();

        for (cost, position) in (1..).zip(iter) {
            result += cost * (position - prev - 1);
            prev = position;
        }

        // Right cost.

        let mut iter = window[middle..].iter().rev().map(i32_as_u32);
        let mut prev = iter.next().unwrap();

        for (cost, position) in (1..).zip(iter) {
            result += cost * (prev - position - 1);
            prev = position;
        }

        result
    }

    pub fn min_moves(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let k = k as u32 as usize;

        Self::find_ones(&mut nums);

        let mut iter = nums
            .iter()
            .zip(&nums[k / 2..])
            .zip(&nums[k - 1..])
            .map(|((&left, &middle), &right)| ((left as u32, middle as u32), right as u32));

        let mut window = iter.next().unwrap().0;
        let mut cost = Self::get_cost(&nums[..k]);
        let mut min_cost = cost;

        if k & 1 == 0 {
            for new_window in iter {
                cost += new_window.1;
                cost -= window.1 * 2 - window.0;

                window = new_window.0;
                min_cost = min_cost.min(cost);
            }
        } else {
            for new_window in iter {
                cost += new_window.1 - new_window.0.1;
                cost -= window.1 - window.0;

                window = new_window.0;
                min_cost = min_cost.min(cost);
            }
        }

        min_cost as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_moves(nums: Vec<i32>, k: i32) -> i32 {
        Self::min_moves(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
