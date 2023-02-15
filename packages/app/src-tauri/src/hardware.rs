use sysinfo::{CpuExt, System, SystemExt};

#[derive(serde::Serialize, Default)]
struct CpuData {
    // 芯片名称
    chip_name: String,
    // 物理核心数
    physical_core_counts: usize,
    // 最大支持线程
    max_threads_counts: usize,
}
impl CpuData {
    pub fn new(sys_info: &System) -> Self {
        Self {
            chip_name: sys_info.global_cpu_info().brand().to_string(),
            physical_core_counts: sys_info.physical_core_count().unwrap(),
            max_threads_counts: sys_info.cpus().len(),
        }
    }
}

#[derive(serde::Serialize, Default)]
pub struct SystemHardwareData {
    cpu: CpuData,
}

#[tauri::command]
pub fn system_hardware_info() -> SystemHardwareData {
    let mut sys = System::default();
    sys.refresh_all();

    SystemHardwareData {
        cpu: CpuData::new(&sys),
    }
}
