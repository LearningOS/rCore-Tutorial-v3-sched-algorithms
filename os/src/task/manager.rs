use super::{ProcessControlBlock, TaskControlBlock};
use crate::console::print;
use crate::sync::UPSafeCell;
use crate::timer::get_time;
use alloc::collections::btree_map::Values;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::sync::Arc;
use alloc::vec::Vec;
use lazy_static::*;
use crate::config::TICKET_X;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
    total_counts: usize,
    lottery_array: Vec<usize>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
            total_counts: 0,
            lottery_array: Vec::new(),
        }
    }
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.total_counts += task.inner_exclusive_access().task_priority * TICKET_X;
        self.lottery_array.push(task.inner_exclusive_access().task_priority);
        self.ready_queue.push_back(task);
    }
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        let mut rng = ChaCha20Rng::seed_from_u64(get_time() as u64);
        let lucky_dog =  rng.gen_range(0..self.total_counts);
        //println!("{}",lucky_dog);
        let mut tmp_sum = 0;
        for (ind, &val) in self.lottery_array.iter().enumerate(){
            tmp_sum += val * TICKET_X;
            if tmp_sum > lucky_dog{
                self.lottery_array.remove(ind);
                self.total_counts -= val * TICKET_X;
                return self.ready_queue.remove(ind);
            }
        }
        self.ready_queue.pop_front()
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
