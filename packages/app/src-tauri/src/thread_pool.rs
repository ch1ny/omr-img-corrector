use once_cell::sync::Lazy;
use std::{collections::VecDeque, sync::Mutex, thread};

type Job = Box<dyn FnOnce() + 'static + Send>;

// 线程池最大并行任务数
static MAX_WORKERS_COUNT: Mutex<usize> = Mutex::new(1);
// 执行中的任务数
static RUNNING_WORKERS_COUNT: Mutex<usize> = Mutex::new(0);
// 等待中的线程队列
static WAITING_WORKERS_QUEUE: Mutex<Lazy<VecDeque<Job>>> =
    Mutex::new(Lazy::new(|| VecDeque::<Job>::new()));

fn before_execute() {
    loop {
        let max_workers_count = MAX_WORKERS_COUNT.lock().unwrap();
        let mut running_workers_count = RUNNING_WORKERS_COUNT.lock().unwrap();
        if *running_workers_count >= *max_workers_count {
            return;
        }
        let mut waiting_workers_queue = WAITING_WORKERS_QUEUE.lock().unwrap();
        let next_worker = waiting_workers_queue.pop_front();
        drop(max_workers_count);
        drop(waiting_workers_queue);
        match next_worker {
            None => {
                return;
            }
            Some(bx) => {
                *running_workers_count += 1;
                drop(running_workers_count);
                execute(bx);
            }
        };
    }
}

fn execute<F>(f: F)
where
    F: FnOnce() + 'static + Send,
{
    thread::spawn(move || {
        f();
        let mut running_workers_count = RUNNING_WORKERS_COUNT.lock().unwrap();
        *running_workers_count -= 1;
        drop(running_workers_count);
        // 准备执行下一个任务
        before_execute();
    });
}

pub fn request_task<F>(f: F)
where
    F: FnOnce() + 'static + Send,
{
    let mut running_workers_count = RUNNING_WORKERS_COUNT.lock().unwrap();
    let max_workers_count = MAX_WORKERS_COUNT.lock().unwrap();
    if *running_workers_count < *max_workers_count {
        *running_workers_count += 1;
        drop(running_workers_count);
        drop(max_workers_count);
        execute(f);
    } else {
        let mut waiting_workers_queue = WAITING_WORKERS_QUEUE.lock().unwrap();
        waiting_workers_queue.push_back(Box::new(f));
    }
}

#[tauri::command]
pub fn set_max_workers_count(count: usize) {
    if count == 0 {
        panic!("The value of `max_workers_count` must be greater than zero!")
    }

    let mut max_workers_count = MAX_WORKERS_COUNT.lock().unwrap();
    *max_workers_count = count;
    // 后面要用到这把锁，不 drop 掉会死锁
    drop(max_workers_count);

    // 改变最大任务数后寻找是否可以运行新的任务
    before_execute();
}
