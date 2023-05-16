use oics::omr;
use serde::Serialize;

use crate::thread_pool;

#[derive(Serialize, Clone)]
struct StartRunningTaskEventPayload {
    task_id: usize,
}

#[derive(Serialize, Clone)]
struct TaskCompletedEventPayload {
    task_id: usize,
    output_path: String,
    result: String,
}

#[tauri::command]
pub fn add_task(
    task_id: usize,
    input_file: String,
    output_file: String,
    projection_max_angle: u16,
    projection_angle_step: f64,
    projection_max_width: i32,
    projection_max_height: i32,
    hough_min_line_length: f64,
    hough_max_line_gap: f64,
    window: tauri::Window,
) {
    thread_pool::request_task(move || {
        window
            .emit(
                "start_running_task",
                StartRunningTaskEventPayload { task_id },
            )
            .unwrap();
        let result = omr::correct_default(
            &input_file,
            &output_file,
            projection_max_angle,
            projection_angle_step,
            projection_max_width,
            projection_max_height,
            hough_min_line_length,
            hough_max_line_gap,
        );
        let task_completed_payload: TaskCompletedEventPayload = match result {
            Ok((_rotate_angle, is_debatable)) => TaskCompletedEventPayload {
                task_id,
                result: String::from(if is_debatable {
                    "debatable"
                } else {
                    "finished"
                }),
                output_path: output_file,
            },
            Err(_) => TaskCompletedEventPayload {
                task_id,
                result: String::from("error"),
                output_path: output_file,
            },
        };
        window
            .emit("task_completed", task_completed_payload)
            .unwrap();
    });
}
