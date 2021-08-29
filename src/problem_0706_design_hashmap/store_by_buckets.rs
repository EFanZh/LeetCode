// ------------------------------------------------------ snip ------------------------------------------------------ //

pub struct MyHashMap {
    buckets: Vec<Vec<(i32, i32)>>,
    length: usize,
}

fn hash_key(key: i32) -> u32 {
    (key as u32).wrapping_mul(0x9E37_79B9)
}

impl MyHashMap {
    fn new() -> Self {
        Self {
            buckets: vec![Vec::new(); 4],
            length: 0,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let num_buckets = self.buckets.len();
        let bucket_bits = num_buckets.trailing_zeros();
        let hash = hash_key(key);
        let bucket = &mut self.buckets[(hash >> (32 - bucket_bits)) as usize];

        if let Some((_, v)) = bucket.iter_mut().find(|(k, _)| *k == key) {
            *v = value;
        } else {
            if (self.length + 1) * 4 > num_buckets * 3 {
                let mut new_buckets = vec![Vec::new(); num_buckets * 2];

                for bucket in &self.buckets {
                    for &(k, v) in bucket {
                        new_buckets[(hash_key(k) >> (31 - bucket_bits)) as usize].push((k, v));
                    }
                }

                new_buckets[(hash >> (31 - bucket_bits)) as usize].push((key, value));

                self.buckets = new_buckets;
            } else {
                bucket.push((key, value));
            }

            self.length += 1;
        }
    }

    fn bucket_index(&self, key: i32) -> usize {
        let bucket_bits = self.buckets.len().trailing_zeros();

        (hash_key(key) >> (32 - bucket_bits)) as _
    }

    fn get(&self, key: i32) -> i32 {
        let bucket_index = self.bucket_index(key);

        self.buckets[bucket_index]
            .iter()
            .find_map(|&(k, v)| if k == key { Some(v) } else { None })
            .unwrap_or(-1)
    }

    fn remove(&mut self, key: i32) {
        let bucket_index = self.bucket_index(key);
        let bucket = &mut self.buckets[bucket_index];

        if let Some(i) = bucket.iter().position(|&(k, _)| k == key) {
            bucket.swap_remove(i);

            self.length -= 1;
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::MyHashMap for MyHashMap {
    fn new() -> Self {
        Self::new()
    }

    fn put(&mut self, key: i32, value: i32) {
        self.put(key, value);
    }

    fn get(&self, key: i32) -> i32 {
        self.get(key)
    }

    fn remove(&mut self, key: i32) {
        self.remove(key);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::MyHashMap>();
    }
}
