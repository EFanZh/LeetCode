// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

struct Node {
    children: [usize; 26],
    has_value: bool,
}

impl Node {
    fn address(&self) -> usize {
        self as *const _ as usize
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            children: [usize::MAX; 26],
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
    transitions: Vec<[usize; 26]>,
    terminal_states: HashSet<usize>,
    current_state: usize,
}

impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        // Build trie.

        let mut pool = vec![Node::default()];
        let mut pool_length = pool.len();

        for word in &words {
            let mut node = 0;

            for c in word.bytes() {
                let child = &mut pool[node].children[usize::from(c) - usize::from(b'a')];

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

            pool[node].has_value = true;
        }

        // Build DFA.

        let root = pool.first().unwrap();
        let mut current_state: Rc<[&Node]> = Rc::new([root]);
        let mut transitions = Vec::new();
        let mut state_ids = HashMap::from([(Rc::clone(&current_state), 0)]);
        let mut terminal_states = HashSet::new();
        let mut queue = VecDeque::new();

        loop {
            let mut transition = [0; 26];

            for (c, next) in transition.iter_mut().enumerate() {
                let mut next_state = vec![root];

                for &sub_state in current_state.iter() {
                    if let Some(child) = pool.get(sub_state.children[c]) {
                        next_state.push(child);
                    }
                }

                if next_state.len() > 1 {
                    next_state.sort_unstable_by_key(|node| node.address());
                    next_state.dedup();

                    let next_state = Rc::from(next_state);
                    let candidate_id = state_ids.len();

                    *next = match state_ids.entry(next_state) {
                        Entry::Occupied(entry) => *entry.get(),
                        Entry::Vacant(entry) => {
                            if entry.key().iter().any(|node| node.has_value) {
                                terminal_states.insert(candidate_id);
                            }

                            queue.push_back(Rc::clone(entry.key()));

                            entry.insert(candidate_id);

                            candidate_id
                        }
                    };
                }
            }

            transitions.push(transition);

            if let Some(next_state) = queue.pop_front() {
                current_state = next_state;
            } else {
                break;
            }
        }

        Self {
            transitions,
            terminal_states,
            current_state: 0,
        }
    }

    fn query(&mut self, letter: char) -> bool {
        self.current_state = self.transitions[self.current_state][u32::from(letter) as usize - usize::from(b'a')];

        self.terminal_states.contains(&self.current_state)
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
