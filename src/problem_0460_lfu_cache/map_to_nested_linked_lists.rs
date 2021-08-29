// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::mem;
use std::ops::{Index, IndexMut};

const INVALID_HANDLE: usize = usize::MAX;

struct Node {
    key: i32,
    value: i32,
    block: usize,
    prev: usize,
    next: usize,
}

struct Nodes {
    nodes: Vec<Node>,
}

impl Nodes {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(capacity),
        }
    }

    fn allocate(&mut self, node: Node) -> usize {
        let result = self.nodes.len();

        self.nodes.push(node);

        result
    }

    fn get_mut(&mut self, handle: usize) -> Option<&mut Node> {
        self.nodes.get_mut(handle)
    }
}

impl Index<usize> for Nodes {
    type Output = Node;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl IndexMut<usize> for Nodes {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}

struct Block {
    frequency: usize,
    node_head: usize,
    node_tail: usize,
    prev: usize,
    next: usize,
}

struct Blocks {
    storage: Vec<Block>,
    free_head: usize,
}

impl Blocks {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            storage: Vec::with_capacity(capacity),
            free_head: INVALID_HANDLE,
        }
    }

    #[allow(clippy::option_if_let_else)]
    fn allocate(&mut self, block: Block) -> usize {
        if let Some(slot) = self.storage.get_mut(self.free_head) {
            let result = mem::replace(&mut self.free_head, slot.next);

            *slot = block;

            result
        } else {
            let handle = self.storage.len();

            self.storage.push(block);

            handle
        }
    }

    fn free(&mut self, handle: usize) {
        self.storage[handle].next = self.free_head;
        self.free_head = handle;
    }

    fn get(&mut self, handle: usize) -> Option<&Block> {
        self.storage.get(handle)
    }

    fn get_mut(&mut self, handle: usize) -> Option<&mut Block> {
        self.storage.get_mut(handle)
    }
}

impl Index<usize> for Blocks {
    type Output = Block;

    fn index(&self, index: usize) -> &Self::Output {
        &self.storage[index]
    }
}

impl IndexMut<usize> for Blocks {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.storage[index]
    }
}

pub struct LFUCache {
    capacity: usize,
    key_to_node: HashMap<i32, usize>,
    node_memory: Nodes,
    block_memory: Blocks,
    block_head: usize,
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        let capacity = capacity as _;

        Self {
            capacity,
            key_to_node: HashMap::with_capacity(capacity),
            node_memory: Nodes::with_capacity(capacity),
            block_memory: Blocks::with_capacity(capacity),
            block_head: INVALID_HANDLE,
        }
    }

    fn remove_block(&mut self, block_index: usize) {
        // Unlink block.

        let block = &mut self.block_memory[block_index];
        let prev_block_index = block.prev;
        let next_block_index = block.next;

        if let Some(prev_block) = self.block_memory.get_mut(prev_block_index) {
            prev_block.next = next_block_index;
        } else {
            self.block_head = next_block_index;
        }

        if let Some(next_block) = self.block_memory.get_mut(next_block_index) {
            next_block.prev = prev_block_index;
        }

        // Free block.

        self.block_memory.free(block_index);
    }

    fn append_to_block(&mut self, node_index: usize, block_index: usize) {
        let mut node = &mut self.node_memory[node_index];
        let mut block = &mut self.block_memory[block_index];

        node.block = block_index;
        node.prev = block.node_tail;
        node.next = INVALID_HANDLE;

        self.node_memory[block.node_tail].next = node_index;

        block.node_tail = node_index;
    }

    fn increment_frequence(&mut self, node_index: usize) {
        let node = &mut self.node_memory[node_index];
        let block = &mut self.block_memory[node.block];
        let frequency = block.frequency;
        let next_block_index = block.next;

        if block.node_head == block.node_tail {
            // The node is the only node in the block.

            if self
                .block_memory
                .get(next_block_index)
                .map_or(false, |next_block| next_block.frequency == frequency + 1)
            {
                // Move the node into the next block, then remove the current block.

                let block_index = node.block;

                self.append_to_block(node_index, next_block_index);
                self.remove_block(block_index);
            } else {
                // Increment the node’s frequency.

                self.block_memory[node.block].frequency += 1;
            }
        } else {
            // Unlink the node from the current block.

            let block_index = node.block;
            let prev_node_index = node.prev;
            let next_node_index = mem::replace(&mut node.next, INVALID_HANDLE);

            if let Some(prev_node) = self.node_memory.get_mut(prev_node_index) {
                prev_node.next = next_node_index;
            } else {
                block.node_head = next_node_index;
            }

            if let Some(next_node) = self.node_memory.get_mut(next_node_index) {
                next_node.prev = prev_node_index;
            } else {
                block.node_tail = prev_node_index;
            }

            if self
                .block_memory
                .get(next_block_index)
                .map_or(false, |next_block| next_block.frequency == frequency + 1)
            {
                // Insert node into the next block.

                self.append_to_block(node_index, next_block_index);
            } else {
                // Create a new block and insert it after the block.

                let new_block_index = self.block_memory.allocate(Block {
                    frequency: frequency + 1,
                    node_head: node_index,
                    node_tail: node_index,
                    prev: block_index,
                    next: next_block_index,
                });

                self.node_memory[node_index].prev = INVALID_HANDLE;
                self.node_memory[node_index].block = new_block_index;
                self.block_memory[block_index].next = new_block_index;

                if let Some(next_block) = self.block_memory.get_mut(next_block_index) {
                    next_block.prev = new_block_index;
                }
            }
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&node_index) = self.key_to_node.get(&key) {
            self.increment_frequence(node_index);

            self.node_memory[node_index].value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(&node_index) = self.key_to_node.get(&key) {
            self.node_memory[node_index].value = value;
            self.increment_frequence(node_index);
        } else if let Some(block) = self.block_memory.get_mut(self.block_head) {
            if self.key_to_node.len() == self.capacity {
                // Reuse the first node.

                let node = &mut self.node_memory[block.node_head];
                let old_key = node.key;
                let node_index = block.node_head;

                node.key = key;
                node.value = value;

                if block.node_head == block.node_tail {
                    // Reuse the block.

                    block.frequency = 1;
                } else {
                    // Unlink the first element.

                    block.node_head = node.next;
                    self.node_memory[block.node_head].prev = INVALID_HANDLE;

                    if block.frequency == 1 {
                        // Append to the block.

                        self.append_to_block(node_index, self.block_head);
                    } else {
                        // Create a new block.

                        let new_block = self.block_memory.allocate(Block {
                            frequency: 1,
                            node_head: node_index,
                            node_tail: node_index,
                            prev: INVALID_HANDLE,
                            next: self.block_head,
                        });

                        self.node_memory[node_index].block = new_block;
                        self.node_memory[node_index].next = INVALID_HANDLE;
                        self.block_memory[self.block_head].prev = new_block;
                        self.block_head = new_block;
                    }
                }

                self.key_to_node.remove(&old_key);
            } else if block.frequency == 1 {
                // Insert the node after the tail.

                let node_index = self.node_memory.allocate(Node {
                    key,
                    value,
                    block: self.block_head,
                    prev: block.node_tail,
                    next: INVALID_HANDLE,
                });

                self.node_memory[block.node_tail].next = node_index;
                block.node_tail = node_index;
            } else {
                // Create a new node and a new block.

                let node_index = self.node_memory.allocate(Node {
                    key,
                    value,
                    block: INVALID_HANDLE,
                    prev: INVALID_HANDLE,
                    next: INVALID_HANDLE,
                });

                let new_block_index = self.block_memory.allocate(Block {
                    frequency: 1,
                    node_head: node_index,
                    node_tail: node_index,
                    prev: INVALID_HANDLE,
                    next: self.block_head,
                });

                self.node_memory[node_index].block = new_block_index;
                self.block_memory[self.block_head].prev = new_block_index;
                self.block_head = new_block_index;
            }

            self.key_to_node
                .insert(key, self.block_memory[self.block_head].node_tail);
        } else if self.capacity != 0 {
            let node_index = self.node_memory.allocate(Node {
                key,
                value,
                block: 0,
                prev: INVALID_HANDLE,
                next: INVALID_HANDLE,
            });

            let block_index = self.block_memory.allocate(Block {
                frequency: 1,
                node_head: node_index,
                node_tail: node_index,
                prev: INVALID_HANDLE,
                next: INVALID_HANDLE,
            });

            self.block_head = block_index;
            self.key_to_node.insert(key, node_index);
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::LFUCache for LFUCache {
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
        super::super::tests::run::<super::LFUCache>();
    }
}
