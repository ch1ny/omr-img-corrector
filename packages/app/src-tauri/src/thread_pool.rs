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
            // 如果当前正在执行的任务数不小于最大任务数，不做任何处理
            return;
        }
        let mut waiting_workers_queue = WAITING_WORKERS_QUEUE.lock().unwrap();
        // 从等待队列中取出第一个任务
        let next_worker = waiting_workers_queue.pop_front();
        drop(max_workers_count);
        drop(waiting_workers_queue);
        match next_worker {
            None => {
                return;
            }
            Some(bx) => {
                // 如果存在待执行的任务则立即执行
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
        // 任务执行完毕后使执行中任务计数减一
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
        // 若当前执行中任务小于最大执行任务则立即执行
        *running_workers_count += 1;
        drop(running_workers_count);
        drop(max_workers_count);
        execute(f);
    } else {
        // 否则将任务添加到等待队列尾部
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
