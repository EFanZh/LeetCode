// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::mem;

const INVALID_HANDLE: usize = usize::MAX;

struct CacheEntry {
    key: i32,
    value: i32,
    prev: usize,
    next: usize,
}

struct LRUCache {
    map: HashMap<i32, usize>,
    memory: Vec<CacheEntry>,
    newest: usize,
    oldest: usize,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::new(),
            memory: Vec::with_capacity(capacity as _),
            newest: INVALID_HANDLE,
            oldest: INVALID_HANDLE,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&handle) = self.map.get(&key) {
            let cache_entry = &mut self.memory[handle];
            let result = cache_entry.value;

            if cache_entry.prev != INVALID_HANDLE {
                // Remove the entry.

                let old_prev = mem::replace(&mut cache_entry.prev, INVALID_HANDLE);
                let old_next = mem::replace(&mut cache_entry.next, self.newest);

                self.memory[old_prev].next = old_next;

                if let Some(next) = self.memory.get_mut(old_next) {
                    next.prev = old_prev;
                } else {
                    self.oldest = old_prev;
                }

                // Insert to newest.

                self.memory[self.newest].prev = handle;
                self.newest = handle;
            }

            result
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let length = self.map.len();

        match self.map.entry(key) {
            Entry::Occupied(entry) => {
                let handle = *entry.get();
                let cache_entry = &mut self.memory[handle];

                cache_entry.value = value;

                if cache_entry.prev != INVALID_HANDLE {
                    // Remove the entry.

                    let old_prev = mem::replace(&mut cache_entry.prev, INVALID_HANDLE);
                    let old_next = mem::replace(&mut cache_entry.next, self.newest);

                    self.memory[old_prev].next = old_next;

                    if let Some(next) = self.memory.get_mut(old_next) {
                        next.prev = old_prev;
                    } else {
                        self.oldest = old_prev;
                    }

                    // Insert to newest.

                    self.memory[self.newest].prev = handle;
                    self.newest = handle;
                }
            }
            Entry::Vacant(entry) => {
                if length == self.memory.capacity() {
                    let handle = self.oldest;

                    // Remove old entry.

                    let cache_entry = &mut self.memory[handle];

                    entry.insert(handle);

                    self.map.remove(&mem::replace(&mut cache_entry.key, key));
                    cache_entry.value = value;

                    let old_prev = mem::replace(&mut cache_entry.prev, INVALID_HANDLE);

                    cache_entry.next = self.newest;

                    if let Some(prev) = self.memory.get_mut(old_prev) {
                        prev.next = INVALID_HANDLE;
                        self.oldest = old_prev;
                    }

                    // Insert to newest.

                    self.memory[self.newest].prev = handle;
                    self.newest = handle;
                } else {
                    let handle = length;

                    self.memory.push(CacheEntry {
                        key,
                        value,
                        prev: INVALID_HANDLE,
                        next: self.newest,
                    });

                    if let Some(newest) = self.memory.get_mut(self.newest) {
                        newest.prev = handle;
                    }

                    self.newest = handle;

                    if self.oldest == INVALID_HANDLE {
                        self.oldest = handle;
                    }

                    entry.insert(handle);
                }
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::LRUCache for LRUCache {
    fn new(capacity: i32) -> Self {
        Self::new(capacity)
    }

    fn get(&mut self, key: i32) -> i32 {
        self.get(key)
    }

    fn put(&mut self, key: i32, value: i32) {
        self.put(key, value);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::LRUCache>();
    }
}
