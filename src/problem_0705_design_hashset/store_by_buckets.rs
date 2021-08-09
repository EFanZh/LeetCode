pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

struct MyHashSet {
    buckets: Vec<Vec<i32>>,
    length: usize,
}

fn hash_key(key: i32) -> u32 {
    (key as u32).wrapping_mul(0x9E37_79B9)
}

impl MyHashSet {
    fn new() -> Self {
        Self {
            buckets: vec![Vec::new(); 4],
            length: 0,
        }
    }

    fn add(&mut self, key: i32) {
        let num_buckets = self.buckets.len();
        let bucket_bits = num_buckets.trailing_zeros();
        let hash = hash_key(key);
        let bucket = &mut self.buckets[(hash >> (32 - bucket_bits)) as usize];

        if !bucket.contains(&key) {
            if (self.length + 1) * 4 > num_buckets * 3 {
                let mut new_buckets = vec![Vec::new(); num_buckets * 2];

                for bucket in &self.buckets {
                    for &k in bucket {
                        new_buckets[(hash_key(k) >> (31 - bucket_bits)) as usize].push(k);
                    }
                }

                new_buckets[(hash >> (31 - bucket_bits)) as usize].push(key);

                self.buckets = new_buckets;
            } else {
                bucket.push(key);
            }

            self.length += 1;
        }
    }

    fn bucket_index(&self, key: i32) -> usize {
        let bucket_bits = self.buckets.len().trailing_zeros();

        (hash_key(key) >> (32 - bucket_bits)) as _
    }

    fn remove(&mut self, key: i32) {
        let bucket_index = self.bucket_index(key);
        let bucket = &mut self.buckets[bucket_index];

        if let Some(i) = bucket.iter().position(|&k| k == key) {
            bucket.swap_remove(i);

            self.length -= 1;
        }
    }

    fn contains(&self, key: i32) -> bool {
        let bucket_index = self.bucket_index(key);

        self.buckets[bucket_index].contains(&key)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::MyHashSet for MyHashSet {
    fn new() -> Self {
        Self::new()
    }

    fn add(&mut self, key: i32) {
        self.add(key);
    }

    fn remove(&mut self, key: i32) {
        self.remove(key);
    }

    fn contains(&self, key: i32) -> bool {
        self.contains(key)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::MyHashSet>();
    }
}
