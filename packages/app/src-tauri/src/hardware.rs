// cSpell: disable-next-line
use sysinfo::{CpuExt, System, SystemExt};

#[derive(serde::Serialize, Default)]
pub struct CpuData {
    // 芯片名称
    chip_name: String,
    // 物理核心数
    physical_core_counts: usize,
    // 最大支持线程
    max_thread_counts: usize,
}
impl CpuData {
    pub fn new(sys_info: &System) -> Self {
        Self {
            chip_name: sys_info.global_cpu_info().brand().to_string(),
            physical_core_counts: sys_info.physical_core_count().unwrap(),
            max_thread_counts: sys_info.cpus().len(),
        }
    }
}

#[tauri::command]
pub fn system_cpu_info() -> CpuData {
    let mut sys = System::default();
    sys.refresh_all();

    CpuData::new(&sys)
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
