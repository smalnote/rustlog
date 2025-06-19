// 实现 SnapshotArray, 初始化为固定长度 n, 支持以下操作
// 设置 index 为 val
// 打快照 snap 保留当前数组状态 snap_id, 从 0 开始递增
// 获取某个快照 snap_id 对应 index 的 val
//  -------------------------------------------------
//  0      x               x            x
//  1                  x   x
//  2            x
//  3
//  4                                   x
//  问题转化为如何高效的记录这样一个矩阵
//  每一个点位用一个(index, val) 表示
//  每个 snapshot 用一组按 index 升序排列的 (index, val) 表示
//  查找时, 从 snap_id 开始,递减 snap_id, 在每一行中进行二分查找,返回每一个找到的,否则就是 0
//
pub struct SnapshotArray {
    length: i32,
    snapshots: Vec<Vec<(i32, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 * Your SnapshotArray object will be instantiated and called as such:
 * let obj = SnapshotArray::new(length);
 * obj.set(index, val);
 * let ret_2: i32 = obj.snap();
 * let ret_3: i32 = obj.get(index, snap_id);
 */
impl SnapshotArray {
    pub fn new(length: i32) -> Self {
        let mut snapshots = Vec::with_capacity(16);
        snapshots.push(Vec::new());
        Self { length, snapshots }
    }

    pub fn set(&mut self, index: i32, val: i32) {
        let snapshot = self.snapshots.last_mut().unwrap();
        match snapshot.binary_search_by(|(i, _)| i.cmp(&index)) {
            Ok(at) => snapshot[at].1 = val,
            Err(to) => snapshot.insert(to, (index, val)),
        }
    }

    pub fn snap(&mut self) -> i32 {
        self.snapshots.push(Vec::new());
        self.snapshots.len() as i32 - 2
    }

    pub fn get(&self, index: i32, snap_id: i32) -> i32 {
        if index >= self.length {
            panic!("index out of range");
        }
        for i in (0..=snap_id as usize).rev() {
            if let Ok(at) = self.snapshots[i].binary_search_by(|(i, _)| i.cmp(&index)) {
                return self.snapshots[i][at].1;
            }
        }
        0
    }
}

#[cfg(test)]
mod test {
    use super::SnapshotArray;
    #[test]
    fn test_example() {
        let mut snapshot_array = SnapshotArray::new(3);
        snapshot_array.set(0, 5);
        let first = snapshot_array.snap();
        snapshot_array.set(0, 6);
        snapshot_array.set(0, 5);
        assert_eq!(snapshot_array.get(first, 0), 5);
    }
}
