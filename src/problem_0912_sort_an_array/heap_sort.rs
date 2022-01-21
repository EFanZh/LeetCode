pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn heap_fix(heap: &mut [i32], mut i: usize) {
        let parent = heap[i];

        loop {
            let left_index = i * 2 + 1;

            if let Some(&left) = heap.get(left_index) {
                let right_index = left_index + 1;

                if let Some(&right) = heap.get(right_index) {
                    let (child_index, child) = if right > left {
                        (right_index, right)
                    } else {
                        (left_index, left)
                    };

                    if child > parent {
                        heap[i] = child;
                        i = child_index;
                    } else {
                        break;
                    }
                } else {
                    if left > parent {
                        heap[i] = left;
                        i = left_index;
                    }

                    break;
                }
            } else {
                break;
            }
        }

        heap[i] = parent;
    }

    fn heap_build(heap: &mut [i32]) {
        for i in (0..heap.len() / 2).rev() {
            Self::heap_fix(heap, i);
        }
    }

    fn heap_sort(nums: &mut [i32]) {
        Self::heap_build(nums);

        for tail in (1..nums.len()).rev() {
            nums.swap(0, tail);
            Self::heap_fix(&mut nums[..tail], 0);
        }
    }

    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;

        Self::heap_sort(&mut nums);

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        Self::sort_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
