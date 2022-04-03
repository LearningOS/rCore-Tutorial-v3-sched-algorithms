use super::{ProcessControlBlock, TaskControlBlock};
use crate::sync::UPSafeCell;
use crate::config::MLFQ_PRIORITY;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::sync::Arc;
use k210_hal::cache::Uncache;
use lazy_static::*;

pub struct TaskManager {
    ready_queue: VecDeque<VecDeque<Arc<TaskControlBlock>>>,
}

/// A MLFQ scheduler.
impl TaskManager {
    pub fn new() -> Self {
        let mut mlfq = VecDeque::with_capacity(MLFQ_PRIORITY);
        let mut tmp = 0;
        while tmp < MLFQ_PRIORITY {
            mlfq.push_back(VecDeque::new());
            tmp += 1;
        }
        Self {
            ready_queue: mlfq
        }
    }
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        let task_inner = task.inner_exclusive_access();
        let priority = task_inner.task_priority;
        drop(task_inner);
        if priority == 0{
            self.ready_queue.get_mut(0).unwrap().push_back(task);
        }
        else{
            self.ready_queue.get_mut(priority - 1).unwrap().push_back(task);
        }
    }
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        for queue in (0..MLFQ_PRIORITY).rev(){
            let mlfq = self.ready_queue.get_mut(queue).unwrap();
            if !mlfq.is_empty() {
                return mlfq.pop_front();
            }
        }
        self.ready_queue.get_mut(MLFQ_PRIORITY - 1).unwrap().pop_front()
    }
}

lazy_static! {
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
    pub static ref PID2PCB: UPSafeCell<BTreeMap<usize, Arc<ProcessControlBlock>>> =
        unsafe { UPSafeCell::new(BTreeMap::new()) };
}

pub fn add_task(task: Arc<TaskControlBlock>) {
    TASK_MANAGER.exclusive_access().add(task);
}

pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    TASK_MANAGER.exclusive_access().fetch()
}

pub fn pid2process(pid: usize) -> Option<Arc<ProcessControlBlock>> {
    let map = PID2PCB.exclusive_access();
    map.get(&pid).map(Arc::clone)
}

pub fn insert_into_pid2process(pid: usize, process: Arc<ProcessControlBlock>) {
    PID2PCB.exclusive_access().insert(pid, process);
}

pub fn remove_from_pid2process(pid: usize) {
    let mut map = PID2PCB.exclusive_access();
    if map.remove(&pid).is_none() {
        panic!("cannot find pid {} in pid2task!", pid);
    }
}
