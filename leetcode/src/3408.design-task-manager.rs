use std::{cmp::Ordering, collections::HashMap};

pub struct TaskManager {
    heap: Vec<Task>,
    // task_id to heap index
    tasks: HashMap<i32, usize>,
}

#[derive(Debug, Eq, PartialEq)]
struct Task {
    user_id: i32,
    task_id: i32,
    priority: i32,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.priority.cmp(&other.priority) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.task_id.cmp(&other.task_id)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
/**
 * Your TaskManager object will be instantiated and called as such:
 * let obj = TaskManager::new(tasks);
 * obj.add(userId, taskId, priority);
 * obj.edit(taskId, newPriority);
 * obj.rmv(taskId);
 * let ret_4: i32 = obj.exec_top();
 */
impl TaskManager {
    pub fn new(tasks: Vec<Vec<i32>>) -> Self {
        let heap: Vec<Task> = tasks
            .into_iter()
            .map(|t| Task {
                user_id: t[0],
                task_id: t[1],
                priority: t[2],
            })
            .collect();
        let tasks = heap
            .iter()
            .enumerate()
            .map(|(index, task)| (task.task_id, index))
            .collect();
        let mut m = Self { heap, tasks };
        m.build_heap();
        m
    }

    fn build_heap(&mut self) {
        for i in (0..=(self.heap.len() - 1) / 2).rev() {
            self.shift_down(i);
        }
    }

    fn shift_down(&mut self, at: usize) -> bool {
        let left = 2 * at + 1;
        if left >= self.heap.len() {
            return false;
        }
        let right = left + 1;
        let max_child = if self.less(left, right) { right } else { left };
        if self.less(at, max_child) {
            self.heap.swap(at, max_child);
            self.tasks
                .entry(self.heap[at].task_id)
                .and_modify(|index| *index = at);
            self.tasks
                .entry(self.heap[max_child].task_id)
                .and_modify(|index| *index = max_child);
            self.shift_down(max_child);
            return true;
        }
        false
    }

    fn less(&self, at: usize, other: usize) -> bool {
        if let Some(at) = self.heap.get(at)
            && let Some(other) = self.heap.get(other)
            && at < other
        {
            true
        } else {
            false
        }
    }

    fn greater(&self, at: usize, other: usize) -> bool {
        if let Some(at) = self.heap.get(at)
            && let Some(other) = self.heap.get(other)
            && at > other
        {
            true
        } else {
            false
        }
    }

    pub fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        let t = Task {
            user_id,
            task_id,
            priority,
        };
        self.heap.push(t);
        self.tasks.insert(task_id, self.heap.len() - 1);
        self.shift_up(self.heap.len() - 1);
    }

    fn shift_up(&mut self, at: usize) -> bool {
        if at == 0 {
            return false;
        }
        let parent = (at - 1) / 2;
        if self.greater(at, parent) {
            self.heap.swap(at, parent);
            self.tasks
                .entry(self.heap[at].task_id)
                .and_modify(|index| *index = at);
            self.tasks
                .entry(self.heap[parent].task_id)
                .and_modify(|index| *index = parent);
            self.shift_up(parent);
            return true;
        }
        false
    }

    pub fn edit(&mut self, task_id: i32, new_priority: i32) {
        if let Some(&at) = self.tasks.get(&task_id)
            && let Some(task) = self.heap.get_mut(at)
        {
            task.priority = new_priority;
            if self.shift_up(at) {
                return;
            }
            self.shift_down(at);
        }
    }

    pub fn rmv(&mut self, task_id: i32) {
        if let Some(&at) = self.tasks.get(&task_id) {
            let last = self.heap.len() - 1;
            if at == last {
                self.heap.pop();
                self.tasks.remove(&task_id);
                return;
            }
            self.heap.swap(at, last);
            self.tasks
                .entry(self.heap[at].task_id)
                .and_modify(|index| *index = at);
            self.heap.pop();
            self.tasks.remove(&task_id);
            if self.shift_up(at) {
                return;
            }
            self.shift_down(at);
        }
    }

    pub fn exec_top(&mut self) -> i32 {
        if self.heap.is_empty() {
            return -1;
        }
        let last = self.heap.len() - 1;
        if self.heap.len() == 1 {
            let task = self.heap.pop().unwrap();
            self.tasks.remove(&task.task_id);
            return task.user_id;
        }
        self.heap.swap(0, last);
        self.tasks
            .entry(self.heap[0].task_id)
            .and_modify(|index| *index = 0);
        let task = self.heap.pop().unwrap();
        self.tasks.remove(&task.task_id);
        self.shift_down(0);
        task.user_id
    }
}

#[cfg(test)]
mod test {
    use super::TaskManager;
    #[test]
    fn test_example() {
        let mut tm = TaskManager::new(vec![vec![1, 101, 10], vec![2, 102, 20], vec![3, 103, 15]]);
        tm.add(4, 104, 5);
        tm.edit(102, 8);
        assert_eq!(tm.exec_top(), 3);
        tm.rmv(101);
        tm.add(5, 105, 15);
        assert_eq!(tm.exec_top(), 5);
    }

    #[test]
    fn test_rmv() {
        let mut tm = TaskManager::new(vec![vec![10, 29, 50], vec![7, 6, 1]]);
        assert_eq!(tm.exec_top(), 10);
        tm.rmv(6);
        tm.add(1, 7, 1);
        assert_eq!(tm.exec_top(), 1);
    }
}
