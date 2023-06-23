// ------------------------------------------------------ snip ------------------------------------------------------ //

const NOT_CALCULATED: u16 = u16::MAX - 1;
const COLUMNS: usize = 16;

pub struct TreeAncestor {
    parents: Box<[u16]>,
}

impl TreeAncestor {
    fn new(n: i32, parent: Vec<i32>) -> Self {
        let _ = n;
        let mut parents = vec![NOT_CALCULATED; COLUMNS * parent.len()].into_boxed_slice();

        for (target, parent) in parents.iter_mut().step_by(COLUMNS).zip(parent) {
            *target = parent as _;
        }

        Self { parents }
    }

    fn get_kth_ancestor(&mut self, node: i32, k: i32) -> i32 {
        fn get_jump(parents: &mut [u16], node: u16, i: u16) -> u16 {
            let address = COLUMNS * usize::from(node) + usize::from(i);
            let mut candidate = parents[address];

            if candidate == NOT_CALCULATED {
                let jump = get_jump(parents, node, i - 1);

                candidate = if jump == u16::MAX {
                    u16::MAX
                } else {
                    get_jump(parents, jump, i - 1)
                };

                parents[address] = candidate;
            }

            candidate
        }

        fn inner(parents: &mut [u16], mut node: u16, mut k: u16) -> u16 {
            loop {
                node = get_jump(parents, node, k.trailing_zeros() as _);

                if node == u16::MAX {
                    break;
                }

                k &= k - 1;

                if k == 0 {
                    break;
                }
            }

            node
        }

        i32::from(inner(&mut self.parents, node as _, k as _) as i16)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::TreeAncestor for TreeAncestor {
    fn new(n: i32, parent: Vec<i32>) -> Self {
        Self::new(n, parent)
    }

    fn get_kth_ancestor(&mut self, node: i32, k: i32) -> i32 {
        self.get_kth_ancestor(node, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::TreeAncestor>();
    }
}
