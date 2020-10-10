use std::cell::Cell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::mem;
use std::rc::{Rc, Weak};

struct Node {
    key: Cell<i32>,
    value: Cell<i32>,
    prev: Cell<Weak<Node>>,
    next: Cell<Weak<Node>>,
}

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, Rc<Node>>,
    newest: Weak<Node>,
    oldest: Weak<Node>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as _,
            map: HashMap::new(),
            newest: Weak::new(),
            oldest: Weak::new(),
        }
    }

    fn refresh(node: &Rc<Node>, newest: &mut Weak<Node>, oldest: &mut Weak<Node>) {
        if let Some(prev) = node.prev.take().upgrade() {
            newest.upgrade().unwrap().prev.set(Rc::downgrade(node));

            let next = node.next.replace(mem::replace(newest, Rc::downgrade(node)));

            if let Some(next) = next.upgrade() {
                next.prev.set(Rc::downgrade(&prev));
            } else {
                *oldest = Rc::downgrade(&prev);
            }

            prev.next.set(next);
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let newest = &mut self.newest;
        let oldest = &mut self.oldest;

        self.map.get(&key).map_or(-1, |node| {
            Self::refresh(node, newest, oldest);

            node.value.get()
        })
    }

    fn put(&mut self, key: i32, value: i32) {
        let length = self.map.len();

        match self.map.entry(key) {
            Entry::Occupied(entry) => {
                let node = entry.get();

                node.value.set(value);

                Self::refresh(node, &mut self.newest, &mut self.oldest);
            }
            Entry::Vacant(entry) => {
                if length == self.capacity {
                    let node = self.oldest.upgrade().unwrap();
                    let old_key = node.key.replace(key);

                    node.value.set(value);

                    if let Some(prev) = node.prev.take().upgrade() {
                        self.newest.upgrade().unwrap().prev.set(Rc::downgrade(&node));

                        node.next.set(mem::replace(&mut self.newest, Rc::downgrade(&node)));

                        self.oldest = Rc::downgrade(&prev);

                        prev.next.take();
                    }

                    entry.insert(node);
                    self.map.remove(&old_key);
                } else {
                    let node = Rc::new(Node {
                        key: Cell::new(key),
                        value: Cell::new(value),
                        prev: Cell::new(Weak::new()),
                        next: Cell::new(self.newest.clone()),
                    });

                    if let Some(next) = mem::replace(&mut self.newest, Rc::downgrade(&node)).upgrade() {
                        next.prev.set(Rc::downgrade(&node));
                    } else {
                        self.oldest = Rc::downgrade(&node);
                    }

                    entry.insert(node);
                }
            }
        }
    }
}

impl super::LRUCache for LRUCache {
    fn new(capacity: i32) -> Self {
        Self::new(capacity)
    }

    fn get(&mut self, key: i32) -> i32 {
        self.get(key)
    }

    fn put(&mut self, key: i32, value: i32) {
        self.put(key, value)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::LRUCache>();
    }
}
