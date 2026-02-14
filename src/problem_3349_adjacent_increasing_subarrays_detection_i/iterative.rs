pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

#[derive(Default)]
struct Window {
    length: u32,
    last: i32,
}

impl Window {
    fn push(&mut self, num: i32) {
        if num > self.last {
            self.length += 1;
        } else {
            self.length = 1;
        }

        self.last = num;
    }
}

impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let k = k.cast_unsigned() as usize;
        let mut window_1 = Window::default();
        let mut window_2 = Window::default();

        for (window, start) in [(&mut window_1, 0), (&mut window_2, k)] {
            for &num in &nums[start..start + k - 1] {
                window.push(num);
            }
        }

        nums[k - 1..].iter().zip(&nums[k * 2 - 1..]).any(|(&num_1, &num_2)| {
            window_1.push(num_1);
            window_2.push(num_2);

            window_1.length >= k as u32 && window_2.length >= k as u32
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        Self::has_increasing_subarrays(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
