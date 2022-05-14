pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

struct Cache {
    data: Box<[u32]>,
    mask: usize,
    length: usize,
}

impl Cache {
    fn new(min_capacity: usize) -> Self {
        let capacity = min_capacity.next_power_of_two();

        Self {
            data: vec![0; capacity].into_boxed_slice(),
            mask: capacity - 1,
            length: 0,
        }
    }

    fn push(&mut self, value: u32) {
        self.data[self.length & self.mask] = value;
        self.length += 1;
    }

    fn get(&self, index: usize) -> u32 {
        self.data[index & self.mask]
    }

    fn last(&self) -> u32 {
        self.get(self.length - 1)
    }
}

impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut cache = Cache::new(k);

        for i in 1..=arr.len() {
            let mut subarray_length = 0;
            let mut max_subarray = 0;
            let mut max = 0;

            for &num in arr[i.saturating_sub(k)..i].iter().rev() {
                let num = num as u32;

                subarray_length += 1;
                max_subarray = max_subarray.max(num);
                max = max.max(cache.get((i - 1).wrapping_sub(subarray_length)) + max_subarray * subarray_length as u32);
            }

            cache.push(max);
        }

        cache.last() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        Self::max_sum_after_partitioning(arr, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
