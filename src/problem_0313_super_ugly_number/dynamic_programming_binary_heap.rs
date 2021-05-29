pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn fix_heap_by_key<T, K: Ord>(heap: &mut [T], mut get_key: impl FnMut(&T) -> K) {
        let mut node_index = 0;
        let node_key = get_key(&heap[node_index]);

        loop {
            let left_index = node_index * 2 + 1;

            if let Some(left_key) = heap.get(left_index).map(&mut get_key) {
                let right_index = left_index + 1;

                if let Some(right_key) = heap.get(right_index).map(&mut get_key) {
                    if right_key < left_key {
                        if right_key < node_key {
                            heap.swap(node_index, right_index);
                            node_index = right_index;
                        } else {
                            break;
                        }
                    } else if left_key < node_key {
                        heap.swap(node_index, left_index);
                        node_index = left_index;
                    } else {
                        break;
                    }
                } else {
                    if left_key < node_key {
                        heap.swap(node_index, left_index);
                    }

                    break;
                }
            } else {
                break;
            }
        }
    }

    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let n = n as _;
        let mut cache = Vec::with_capacity(n);
        let mut indices = vec![0; primes.len()];
        let mut heap = (0..primes.len()).collect::<Vec<_>>();
        let mut prev = 1;

        cache.push(1);

        while cache.len() != n {
            loop {
                let top = heap[0];
                let value = cache[indices[top]] * primes[top];

                indices[top] += 1;

                if value == prev {
                    Self::fix_heap_by_key(&mut heap, |&i| cache[indices[i]] * primes[i]);
                } else {
                    cache.push(value);

                    Self::fix_heap_by_key(&mut heap, |&i| cache[indices[i]] * primes[i]);

                    prev = value;

                    break;
                }
            }
        }

        prev
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        Self::nth_super_ugly_number(n, primes)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
