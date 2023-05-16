use once_cell::sync::Lazy;
use std::{collections::VecDeque, sync::Mutex, thread};

use crate::types::{Method, Params};

struct JobParams {
    method: Method,
    params: Params,
    input: String,
    output: String,
}

// 线程池最大并行任务数
static MAX_WORKERS_COUNT: Mutex<usize> = Mutex::new(1);
// 执行中的任务数
static RUNNING_WORKERS_COUNT: Mutex<usize> = Mutex::new(0);
// 等待中的线程队列
static WAITING_WORKERS_QUEUE: Mutex<Lazy<VecDeque<JobParams>>> =
    Mutex::new(Lazy::new(|| VecDeque::<JobParams>::new()));

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
        let next_worker: Option<JobParams> = waiting_workers_queue.pop_front();
        drop(max_workers_count);
        drop(waiting_workers_queue);
        match next_worker {
            None => {
                std::process::exit(0);
            }
            Some(bx) => {
                // 如果存在待执行的任务则立即执行
                *running_workers_count += 1;
                drop(running_workers_count);
                execute(move || {
                    let _ = run_task(bx.method, bx.params, &bx.input, &bx.output);
                });
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

fn run_task(method: Method, params: Params, input: &str, output: &str) -> oics::OpenCV_Result<()> {
    match method {
        Method::Default => {
            let projection_params = params.get_projection_params().as_ref().unwrap();
            let edges_params = params.get_edges_params().as_ref().unwrap();
            oics::omr::correct_default(
                input,
                output,
                projection_params.projection_max_angle,
                projection_params.projection_angle_step,
                projection_params.projection_max_width,
                projection_params.projection_max_height,
                edges_params.min_line_length,
                edges_params.max_line_gap,
            )?;
        }
        Method::ProjectionOnly => todo!(),
        Method::EdgesDetectionOnly => todo!(),
        Method::FourierTransformOnly => todo!(),
    };

    Ok(())
}

pub fn new_task(method: Method, params: Params, input: String, output: String) {
    let mut running_workers_count = RUNNING_WORKERS_COUNT.lock().unwrap();
    let max_workers_count = MAX_WORKERS_COUNT.lock().unwrap();
    if *running_workers_count < *max_workers_count {
        // 若当前执行中任务小于最大执行任务则立即执行
        *running_workers_count += 1;
        drop(running_workers_count);
        drop(max_workers_count);
        execute(move || {
            let _ = run_task(method, params, &input, &output);
        });
    } else {
        // 否则将任务添加到等待队列尾部
        let mut waiting_workers_queue = WAITING_WORKERS_QUEUE.lock().unwrap();
        waiting_workers_queue.push_back(*Box::new(JobParams {
            method,
            params,
            input,
            output,
        }));
    }
}
