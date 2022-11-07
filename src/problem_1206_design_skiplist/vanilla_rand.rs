// ------------------------------------------------------ snip ------------------------------------------------------ //

use rand::rngs::StdRng;
use rand::{Rng as _, SeedableRng};
use std::cmp::Ordering;

struct Node {
    value: i32,
    right: usize,
    bottom: usize,
}

impl Node {
    fn new(value: i32, right: usize, bottom: usize) -> Self {
        Self { value, right, bottom }
    }
}

struct Allocator {
    memory: Vec<Node>,
    free_head: usize,
}

impl Allocator {
    fn new() -> Self {
        Self {
            memory: Vec::new(),
            free_head: usize::MAX,
        }
    }

    fn allocate(&mut self, node: Node) -> usize {
        let memory = &mut self.memory;
        let candidate = self.free_head;

        if let Some(slot) = memory.get_mut(candidate) {
            let new_free_head = slot.right;

            *slot = node;

            self.free_head = new_free_head;

            candidate
        } else {
            let result = memory.len();

            memory.push(node);

            result
        }
    }

    fn free(&mut self, address: usize) {
        let slot = &mut self.memory[address];

        slot.right = self.free_head;

        self.free_head = address;
    }
}

struct Rng {
    state: StdRng,
}

impl Rng {
    fn new() -> Self {
        Self {
            state: StdRng::from_seed([0; 32]),
        }
    }

    fn random_bool(&mut self) -> bool {
        self.state.gen()
    }
}

pub struct Skiplist {
    head_address: usize,
    allocator: Allocator,
    rng: Rng,
}

impl Skiplist {
    fn new() -> Self {
        Self {
            head_address: usize::MAX,
            allocator: Allocator::new(),
            rng: Rng::new(),
        }
    }

    fn search_helper(&self, value: i32) -> Option<usize> {
        let memory = self.allocator.memory.as_slice();
        let mut left_address = self.head_address;

        while let Some(mut left_node) = memory.get(left_address) {
            loop {
                let address = left_node.right;

                match memory.get(address).and_then(|node| match value.cmp(&node.value) {
                    Ordering::Less => None,
                    Ordering::Equal => Some(None),
                    Ordering::Greater => Some(Some(node)),
                }) {
                    None => {
                        left_address = left_node.bottom;

                        break;
                    }
                    Some(None) => return Some(left_address),
                    Some(Some(node)) => {
                        left_address = address;
                        left_node = node;
                    }
                };
            }
        }

        None
    }

    fn search(&self, target: i32) -> bool {
        self.search_helper(target).is_some()
    }

    fn add(&mut self, num: i32) {
        fn helper(allocator: &mut Allocator, rng: &mut Rng, mut left_address: usize, value: i32) -> Option<usize> {
            // Find the node to insert after.

            let mut left_node = &allocator.memory[left_address];

            while let Some(right_node) = allocator
                .memory
                .get(left_node.right)
                .filter(|right_node| value > right_node.value)
            {
                left_address = left_node.right;
                left_node = right_node;
            }

            let (left_right, left_bottom) = (left_node.right, left_node.bottom);

            if left_bottom == usize::MAX {
                // This is the bottom level.

                let node_address = allocator.allocate(Node::new(value, left_right, usize::MAX));

                allocator.memory[left_address].right = node_address;

                Some(node_address)
            } else {
                // Try to insert into bottom level.

                let bottom_address = helper(allocator, rng, left_bottom, value)?;

                // Try to insert into current level.

                rng.random_bool().then(|| {
                    let node_address = allocator.allocate(Node::new(value, left_right, bottom_address));

                    allocator.memory[left_address].right = node_address;

                    node_address
                })
            }
        }

        let Self {
            head_address,
            allocator,
            rng,
        } = self;

        let head_address = *head_address;

        let address = if head_address == usize::MAX {
            usize::MAX
        } else if let Some(address) = helper(allocator, rng, head_address, num).filter(|_| rng.random_bool()) {
            address
        } else {
            return;
        };

        let first_address = self.allocator.allocate(Node::new(num, usize::MAX, address));
        let head_address = self.allocator.allocate(Node::new(0, first_address, head_address));

        self.head_address = head_address;
    }

    fn erase(&mut self, num: i32) -> bool {
        if let Some(mut left_address) = self.search_helper(num) {
            let allocator = &mut self.allocator;
            let mut left_node = &allocator.memory[left_address];

            loop {
                // Remove `left_node` with address `left_address`.

                let left_bottom_address = left_node.bottom;
                let mut address = left_node.right;
                let node = &allocator.memory[address];
                let bottom_address = node.bottom;
                let right_address = node.right;

                allocator.memory[left_address].right = right_address;
                allocator.free(address);

                left_address = left_bottom_address;
                address = bottom_address;

                if let Some(mut new_left_node) = allocator.memory.get(left_address) {
                    while new_left_node.right != address {
                        left_address = new_left_node.right;
                        new_left_node = &allocator.memory[left_address];
                    }

                    left_node = new_left_node;
                } else {
                    break;
                }
            }

            let head_address = self.head_address;
            let head_node = &allocator.memory[head_address];

            if head_node.right == usize::MAX {
                self.head_address = head_node.bottom;
                allocator.free(head_address);
            }

            true
        } else {
            false
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Skiplist for Skiplist {
    fn new() -> Self {
        Self::new()
    }

    fn search(&self, target: i32) -> bool {
        self.search(target)
    }

    fn add(&mut self, num: i32) {
        self.add(num);
    }

    fn erase(&mut self, num: i32) -> bool {
        self.erase(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Skiplist>();
    }
}
