// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::binary_heap::PeekMut;
use std::collections::hash_map::{Entry, OccupiedEntry};
use std::collections::{BinaryHeap, HashMap};
use std::rc::Rc;

struct Item {
    expiration_time: u32,
    token_id: Rc<str>,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Item {}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        other.expiration_time.cmp(&self.expiration_time)
    }
}

pub struct AuthenticationManager {
    time_to_live: u32,
    heap: BinaryHeap<Item>,
    entries: HashMap<Rc<str>, Cell<u32>>,
}

impl AuthenticationManager {
    fn new(time_to_live: i32) -> Self {
        Self {
            time_to_live: time_to_live as _,
            heap: BinaryHeap::new(),
            entries: HashMap::new(),
        }
    }

    fn generate(&mut self, token_id: String, current_time: i32) {
        let token_id = Rc::from(token_id);
        let current_time = current_time as u32;
        let expiration_time = current_time + self.time_to_live;

        self.heap.push(Item {
            expiration_time,
            token_id: Rc::clone(&token_id),
        });

        self.entries.insert(token_id, Cell::new(expiration_time));
    }

    fn occupied(entry: Entry<Rc<str>, Cell<u32>>) -> Option<OccupiedEntry<Rc<str>, Cell<u32>>> {
        match entry {
            Entry::Occupied(entry) => Some(entry),
            Entry::Vacant(_) => None,
        }
    }

    fn renew(&mut self, token_id: String, current_time: i32) {
        let current_time = current_time as u32;

        if let Some((key, value)) = self.entries.get_key_value(token_id.as_str()) {
            if value.get() > current_time {
                let expiration_time = current_time + self.time_to_live;

                value.set(expiration_time);

                self.heap.push(Item {
                    expiration_time,
                    token_id: Rc::clone(key),
                });
            }
        }
    }

    fn advance(&mut self, current_time: u32) {
        while let Some(first) = self.heap.peek_mut() {
            if first.expiration_time <= current_time {
                let first = PeekMut::pop(first);
                let entry = Self::occupied(self.entries.entry(first.token_id)).unwrap();

                if first.expiration_time == entry.get().get() {
                    entry.remove();
                }
            } else {
                break;
            }
        }
    }

    fn count_unexpired_tokens(&mut self, current_time: i32) -> i32 {
        let current_time = current_time as u32;

        self.advance(current_time);

        self.entries.len() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::AuthenticationManager for AuthenticationManager {
    fn new(time_to_live: i32) -> Self {
        Self::new(time_to_live)
    }

    fn generate(&mut self, token_id: String, current_time: i32) {
        self.generate(token_id, current_time);
    }

    fn renew(&mut self, token_id: String, current_time: i32) {
        self.renew(token_id, current_time);
    }

    fn count_unexpired_tokens(&mut self, current_time: i32) -> i32 {
        self.count_unexpired_tokens(current_time)
    }
}

#[cfg(test)]
mod tests {
    use super::Item;
    use std::cell::Cell;
    use std::collections::HashMap;
    use std::rc::Rc;

    #[test]
    fn test_item_partial_eq() {
        assert!(
            Item {
                expiration_time: 2,
                token_id: Rc::from("foo")
            } == Item {
                expiration_time: 2,
                token_id: Rc::from("bar")
            },
        );
    }

    #[test]
    fn test_occupied() {
        let key = Rc::<str>::from("foo");
        let mut map = HashMap::from([(Rc::clone(&key), Cell::new(2))]);

        assert!(super::AuthenticationManager::occupied(map.entry(key)).is_some());
        assert!(super::AuthenticationManager::occupied(map.entry(Rc::from("bar"))).is_none());
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::AuthenticationManager>();
    }
}
