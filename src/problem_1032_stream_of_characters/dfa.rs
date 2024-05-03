// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, Hasher};
use std::ptr;
use std::rc::Rc;

struct Node {
    children: [u16; 26],
    has_value: bool,
}

impl Node {
    fn address(&self) -> usize {
        ptr::from_ref(self) as _
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            children: [u16::MAX; 26],
            has_value: false,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.address() == other.address()
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.address().hash(state);
    }
}

pub struct StreamChecker {
    states: Vec<([u32; 26], bool)>,
    current_state: u32,
}

impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        // Build trie.

        let mut pool = vec![Node::default()];
        let mut pool_length = 1;

        for word in &words {
            let mut node = 0;

            for c in word.bytes() {
                let child = &mut pool[usize::from(node)].children[usize::from(c) - usize::from(b'a')];

                node = if *child < pool_length {
                    *child
                } else {
                    *child = pool_length;

                    let child = *child;

                    pool.push(Node::default());
                    pool_length += 1;

                    child
                };
            }

            pool[usize::from(node)].has_value = true;
        }

        // Build DFA.

        let root = pool.first().unwrap();
        let mut current_state: Rc<[&Node]> = Rc::new([root]);
        let mut next_state_buffer = Vec::new();
        let mut state_ids = HashMap::<Rc<[_]>, _>::new();
        let mut queue = VecDeque::new();
        let mut states = Vec::new();

        loop {
            let mut transitions = [0; 26];

            for (c, next) in transitions.iter_mut().enumerate() {
                for &sub_state in &*current_state {
                    if let Some(child) = pool.get(usize::from(sub_state.children[c])) {
                        next_state_buffer.push(child);
                    }
                }

                if !next_state_buffer.is_empty() {
                    next_state_buffer.push(root);
                    next_state_buffer.sort_unstable_by_key(|node| node.address());
                    next_state_buffer.dedup();

                    let next_state = Rc::from(next_state_buffer.as_slice());

                    next_state_buffer.clear();

                    let candidate_id = state_ids.len() as u32 + 1;

                    *next = match state_ids.entry(next_state) {
                        Entry::Occupied(entry) => *entry.get(),
                        Entry::Vacant(entry) => {
                            queue.push_back(Rc::clone(entry.key()));

                            entry.insert(candidate_id);

                            candidate_id
                        }
                    };
                }
            }

            states.push((transitions, current_state.iter().any(|node| node.has_value)));

            if let Some(next_state) = queue.pop_front() {
                current_state = next_state;
            } else {
                break;
            }
        }

        Self {
            states,
            current_state: 0,
        }
    }

    fn query(&mut self, letter: char) -> bool {
        self.current_state = self.states[self.current_state as usize].0[u32::from(letter) as usize - usize::from(b'a')];

        self.states[self.current_state as usize].1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::StreamChecker for StreamChecker {
    fn new(words: Vec<String>) -> Self {
        Self::new(words)
    }

    fn query(&mut self, letter: char) -> bool {
        self.query(letter)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::StreamChecker>();
    }
}
