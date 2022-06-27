// ------------------------------------------------------ snip ------------------------------------------------------ //

pub struct SnapshotArray {
    data: Box<[Vec<(i32, u32)>]>,
    version: u32,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        Self {
            data: vec![Vec::new(); length as _].into_boxed_slice(),
            version: 0,
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        let slot = &mut self.data[index as usize];
        let current_version = self.version;

        if let Some(last_val) = slot
            .last_mut()
            .and_then(|(last_val, last_version)| (*last_version == current_version).then(|| last_val))
        {
            *last_val = val;
        } else {
            slot.push((val, self.version));
        }
    }

    fn snap(&mut self) -> i32 {
        let result = self.version;

        self.version += 1;

        result as _
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let snap_id = snap_id as _;
        let slot = self.data[index as usize].as_slice();
        let i = slot.partition_point(|&(_, version)| version <= snap_id).wrapping_sub(1);

        slot.get(i).map_or(0, |&(val, _)| val)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::SnapshotArray for SnapshotArray {
    fn new(length: i32) -> Self {
        Self::new(length)
    }

    fn set(&mut self, index: i32, val: i32) {
        self.set(index, val);
    }

    fn snap(&mut self) -> i32 {
        self.snap()
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        self.get(index, snap_id)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::SnapshotArray>();
    }
}
