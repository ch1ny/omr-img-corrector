use oics::omr;

#[tauri::command]
pub fn add_task(
    task_id: usize,
    input_file: String,
    output_file: String,
    projection_max_angle: u16,
    projection_angle_step: f64,
    projection_resize_scale: f64,
    hough_min_line_length: f64,
    hough_max_line_gap: f64,
) {
    let result = omr::correct_default(
        &input_file,
        &output_file,
        projection_max_angle,
        projection_angle_step,
        projection_resize_scale,
        hough_min_line_length,
        hough_max_line_gap,
    );
}
