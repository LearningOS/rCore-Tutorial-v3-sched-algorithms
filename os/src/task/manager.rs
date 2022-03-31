use super::{ProcessControlBlock, TaskControlBlock};
use crate::sync::UPSafeCell;
use crate::timer::get_time_ms;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::sync::Arc;
use lazy_static::*;

pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple SPN scheduler.
impl TaskManager {
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        let task_inner = task.inner_exclusive_access();
        let prediction = task_inner.task_prediction;
        let running = task_inner.task_isrunning;
        let waiting_time = task_inner.task_waiting_time + get_time_ms() - task_inner.task_last_yield_time;
        // if prediction != 1000000{
        //     println!("{}", prediction);
        // }
        drop(task_inner);
        for queue in 0..self.ready_queue.len(){
            let task1 = self.ready_queue.get(queue).unwrap();
            let task1_inner = task1.inner_exclusive_access();
            let prediction1 = task1_inner.task_prediction;
            let running1 = task1_inner.task_isrunning;
            let waiting_time1 = task1_inner.task_waiting_time + get_time_ms() - task1_inner.task_last_yield_time;
            drop(task1_inner);
            // println!("{}, {}, {}", running, queue, running1);
            if running && !running1{
                self.ready_queue.insert(queue, task);
                return
            }
            else if !running && running1{
                continue;
            }
            else{
                // println!("1 {},{}, 2 {},{}", prediction, waiting_time, prediction1, waiting_time1);
                if (waiting_time * prediction1) > (waiting_time1 * prediction) {
                    self.ready_queue.insert(queue, task);
                    return
                }
            }
        }
        self.ready_queue.push_back(task);
    }
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
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
