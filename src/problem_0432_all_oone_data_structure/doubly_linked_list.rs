// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::{iter, mem};

const INVALID_HANDLE: usize = usize::MAX;

struct Node {
    keys: HashSet<Rc<str>>,
    value: usize,
    prev: usize,
    next: usize,
}

pub struct AllOne {
    nodes: Vec<Node>,
    free_nodes: usize,
    key_to_node: HashMap<Rc<str>, usize>,
    head: usize,
    tail: usize,
}

impl AllOne {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            free_nodes: INVALID_HANDLE,
            key_to_node: HashMap::new(),
            head: INVALID_HANDLE,
            tail: INVALID_HANDLE,
        }
    }

    #[allow(clippy::option_if_let_else)]
    fn allocate_node(&mut self, key: Rc<str>, value: usize, prev: usize, next: usize) -> usize {
        if let Some(free_node) = self.nodes.get_mut(self.free_nodes) {
            let handle = mem::replace(&mut self.free_nodes, free_node.next);

            free_node.keys.insert(key);
            free_node.value = value;
            free_node.prev = prev;
            free_node.next = next;

            handle
        } else {
            let handle = self.nodes.len();

            self.nodes.push(Node {
                keys: iter::once(key).collect(),
                value,
                prev,
                next,
            });

            handle
        }
    }

    fn free_node(&mut self, handle: usize) {
        self.nodes[handle].next = self.free_nodes;
        self.free_nodes = handle;
    }

    fn inc(&mut self, key: String) {
        if let Some(&handle) = self.key_to_node.get(key.as_str()) {
            let node = &self.nodes[handle];
            let value = node.value;
            let prev_handle = node.prev;
            let next_handle = node.next;

            if node.keys.len() == 1 {
                if let Some(next_node) = self.nodes.get(next_handle) {
                    if next_node.value == value + 1 {
                        let rc_key = self.nodes[handle].keys.drain().next().unwrap();
                        let next_node = &mut self.nodes[next_handle];

                        next_node.keys.insert(rc_key);
                        next_node.prev = prev_handle;

                        if let Some(prev_node) = self.nodes.get_mut(prev_handle) {
                            prev_node.next = next_handle;
                        } else {
                            self.head = next_handle;
                        }

                        *self.key_to_node.get_mut(key.as_str()).unwrap() = next_handle;

                        self.free_node(handle);
                    } else {
                        self.nodes[handle].value += 1;
                    }
                } else {
                    self.nodes[handle].value += 1;
                }
            } else {
                let rc_key = self.nodes[handle].keys.take(key.as_str()).unwrap();

                if let Some(next_node) = self.nodes.get_mut(next_handle) {
                    if next_node.value == value + 1 {
                        next_node.keys.insert(rc_key);

                        *self.key_to_node.get_mut(key.as_str()).unwrap() = next_handle;
                    } else {
                        let new_handle = self.allocate_node(rc_key, value + 1, handle, next_handle);

                        self.nodes[handle].next = new_handle;
                        self.nodes[next_handle].prev = new_handle;
                        *self.key_to_node.get_mut(key.as_str()).unwrap() = new_handle;
                    }
                } else {
                    let new_handle = self.allocate_node(rc_key, value + 1, handle, next_handle);

                    self.nodes[handle].next = new_handle;
                    self.tail = new_handle;
                    *self.key_to_node.get_mut(key.as_str()).unwrap() = new_handle;
                }
            }
        } else {
            let rc_key = Rc::<str>::from(key);

            if let Some(first_node) = self.nodes.get_mut(self.head) {
                if first_node.value == 0 {
                    first_node.keys.insert(Rc::clone(&rc_key));
                } else {
                    let handle = self.allocate_node(Rc::clone(&rc_key), 0, INVALID_HANDLE, self.head);

                    self.nodes[self.head].prev = handle;
                    self.head = handle;
                }
            } else {
                let handle = self.allocate_node(Rc::clone(&rc_key), 0, INVALID_HANDLE, INVALID_HANDLE);

                self.head = handle;
                self.tail = handle;
            }

            self.key_to_node.insert(rc_key, self.head);
        }
    }

    fn dec(&mut self, key: String) {
        if let Some(&handle) = self.key_to_node.get(key.as_str()) {
            let node = &mut self.nodes[handle];
            let value = node.value;
            let prev_handle = node.prev;
            let next_handle = node.next;

            if value == 0 {
                node.keys.remove(key.as_str());

                if node.keys.is_empty() {
                    self.head = next_handle;

                    if let Some(next_node) = self.nodes.get_mut(next_handle) {
                        next_node.prev = prev_handle;
                    } else {
                        self.tail = prev_handle;
                    }

                    self.free_node(handle);
                }

                self.key_to_node.remove(key.as_str());
            } else if node.keys.len() == 1 {
                if let Some(prev_node) = self.nodes.get(prev_handle) {
                    if prev_node.value + 1 == value {
                        let rc_key = self.nodes[handle].keys.drain().next().unwrap();
                        let prev_node = &mut self.nodes[prev_handle];

                        prev_node.keys.insert(rc_key);
                        prev_node.next = next_handle;

                        if let Some(next_node) = self.nodes.get_mut(next_handle) {
                            next_node.prev = prev_handle;
                        } else {
                            self.tail = prev_handle;
                        }

                        *self.key_to_node.get_mut(key.as_str()).unwrap() = prev_handle;

                        self.free_node(handle);
                    } else {
                        self.nodes[handle].value -= 1;
                    }
                } else {
                    self.nodes[handle].value -= 1;
                }
            } else {
                let rc_key = self.nodes[handle].keys.take(key.as_str()).unwrap();

                if let Some(prev_node) = self.nodes.get_mut(prev_handle) {
                    if prev_node.value + 1 == value {
                        prev_node.keys.insert(rc_key);

                        *self.key_to_node.get_mut(key.as_str()).unwrap() = prev_handle;
                    } else {
                        let new_handle = self.allocate_node(rc_key, value - 1, prev_handle, handle);

                        self.nodes[handle].prev = new_handle;
                        self.nodes[prev_handle].next = new_handle;
                        *self.key_to_node.get_mut(key.as_str()).unwrap() = new_handle;
                    }
                } else {
                    let new_handle = self.allocate_node(rc_key, value - 1, prev_handle, handle);

                    self.nodes[handle].prev = new_handle;
                    self.head = new_handle;
                    *self.key_to_node.get_mut(key.as_str()).unwrap() = new_handle;
                }
            }
        }
    }

    fn get_max_key(&self) -> String {
        self.nodes.get(self.tail).map_or_else(String::new, |node| {
            node.keys.iter().next().as_deref().unwrap().to_string()
        })
    }

    fn get_min_key(&self) -> String {
        self.nodes.get(self.head).map_or_else(String::new, |node| {
            node.keys.iter().next().as_deref().unwrap().to_string()
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::AllOne for AllOne {
    fn new() -> Self {
        Self::new()
    }

    fn inc(&mut self, key: String) {
        self.inc(key);
    }

    fn dec(&mut self, key: String) {
        self.dec(key);
    }

    fn get_max_key(&self) -> String {
        self.get_max_key()
    }

    fn get_min_key(&self) -> String {
        self.get_min_key()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::AllOne>();
    }
}
