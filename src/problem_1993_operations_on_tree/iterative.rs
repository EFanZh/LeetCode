// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

struct Node {
    parent: u16,
    locking_user: u16,
    locked_descendants: HashSet<u16>,
}

pub struct LockingTree {
    nodes: Box<[Node]>,
    unlock_queue: Vec<u16>,
}

impl LockingTree {
    fn new(parent: Vec<i32>) -> Self {
        Self {
            nodes: parent
                .into_iter()
                .map(|parent| Node {
                    parent: parent as _,
                    locking_user: u16::MAX,
                    locked_descendants: HashSet::new(),
                })
                .collect(),
            unlock_queue: Vec::new(),
        }
    }

    fn for_all_ancestors(nodes: &mut [Node], mut index: u16, mut f: impl FnMut(&mut HashSet<u16>)) {
        while let Some(node) = nodes.get_mut(usize::from(index)) {
            index = node.parent;
            f(&mut node.locked_descendants);
        }
    }

    fn insert_item(num: u16) -> impl FnMut(&mut HashSet<u16>) {
        move |set| {
            set.insert(num);
        }
    }

    fn lock_helper(
        nodes: &mut [Node],
        num: u16,
        expected: u16,
        replace: u16,
        f: impl FnMut(&mut HashSet<u16>),
    ) -> bool {
        let node = &mut nodes[usize::from(num)];
        let can_operate = node.locking_user == expected;

        if can_operate {
            node.locking_user = replace;

            let parent_index = node.parent;

            Self::for_all_ancestors(nodes, parent_index, f);
        }

        can_operate
    }

    fn remove_item(num: u16) -> impl FnMut(&mut HashSet<u16>) {
        move |set| {
            set.remove(&num);
        }
    }

    fn lock(&mut self, num: i32, user: i32) -> bool {
        let num = num as u16;

        Self::lock_helper(&mut self.nodes, num, u16::MAX, user as _, Self::insert_item(num))
    }

    fn unlock(&mut self, num: i32, user: i32) -> bool {
        let num = num as u16;

        Self::lock_helper(&mut self.nodes, num, user as _, u16::MAX, Self::remove_item(num))
    }

    fn upgrade(&mut self, num: i32, user: i32) -> bool {
        let Self { nodes, unlock_queue } = self;
        let num = usize::from(num as u16);
        let nodes = nodes.as_mut();
        let node = &nodes[num];

        // Check update condition.
        {
            if node.locking_user != u16::MAX || node.locked_descendants.is_empty() {
                return false;
            }

            let mut node = node;

            loop {
                let parent_index = node.parent;

                if let Some(parent_node) = nodes.get(usize::from(parent_index)) {
                    if parent_node.locking_user != u16::MAX {
                        return false;
                    }

                    node = parent_node;
                } else {
                    break;
                }
            }
        }

        // Unlock descendants.

        unlock_queue.extend(&node.locked_descendants);

        while let Some(descendant) = unlock_queue.pop() {
            let node = &mut nodes[usize::from(descendant)];

            node.locking_user = u16::MAX;

            let parent_index = node.parent;

            Self::for_all_ancestors(nodes, parent_index, Self::remove_item(descendant));
        }

        // Lock self.

        let node = &mut nodes[num];

        node.locking_user = user as _;

        let parent_index = node.parent;

        Self::for_all_ancestors(nodes, parent_index, Self::insert_item(num as _));

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::LockingTree for LockingTree {
    fn new(parent: Vec<i32>) -> Self {
        Self::new(parent)
    }

    fn lock(&mut self, num: i32, user: i32) -> bool {
        self.lock(num, user)
    }

    fn unlock(&mut self, num: i32, user: i32) -> bool {
        self.unlock(num, user)
    }

    fn upgrade(&mut self, num: i32, user: i32) -> bool {
        self.upgrade(num, user)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::LockingTree>();
    }
}
