use sysinfo::{ProcessExt, System, SystemExt};
use std::{thread, time};
use std::io::{Write, stdout};
use crossterm::{QueueableCommand, cursor};

use nvml_wrapper::{
    NVML,
};
use nvml_wrapper::enum_wrappers::device::{
    Clock,
    TemperatureSensor,
};
use nvml_wrapper::enums::device::UsedGpuMemory;
use nvml_wrapper::error::NvmlError as NvmlError;
use nvml_wrapper::device::*;
use nvml_wrapper::structs::device::*;
use nvml_wrapper::struct_wrappers::device::*;

pub struct GPUstat {
    name: Result<String, NvmlError>,
    id: Result<u32, NvmlError>,
    compute_capability: Result<CudaComputeCapability, NvmlError>,
    utilization_rates: Result<Utilization, NvmlError>,
    memory_info: Result<MemoryInfo, NvmlError>,
    fan_speed: Result<u32, NvmlError>,
    temperature: Result<u32, NvmlError>,
    running_graphics_processes: Result<Vec<ProcessInfo>, NvmlError>,
}

pub fn read_gpu_stat(device: &nvml_wrapper::Device) -> GPUstat {
    let gpustat = GPUstat {
        name: device.name(),
        id: device.index(),
        compute_capability: device.cuda_compute_capability(),
        utilization_rates: device.utilization_rates(),
        memory_info: device.memory_info(),
        fan_speed: device.fan_speed(0), // Currently only take one fan, will add more fan readings
        temperature: device.temperature(TemperatureSensor::Gpu),
        running_graphics_processes: device.running_graphics_processes(),
    };

    return gpustat;
}

pub fn dump_gpu_stat(device: nvml_wrapper::Device) {
    let gpustat = read_gpu_stat(&device);
    let mut result = "".to_owned();

    let id = match gpustat.id {
        Ok(id) => format!("[{}]: ", id),
        Err(_err) => "".to_string(),
    };
    result.push_str(&id);
    
    let name = match gpustat.name {
        Ok(name) => format!("{} | ", name),
        Err(_err) => "".to_string(),
    };
    result.push_str(&name);

    let compute_capability = match gpustat.compute_capability {
        Ok(compute_capability) => format!("{}{} | ", compute_capability.major, compute_capability.minor),
        Err(_err) => "".to_string(),
    };
    result.push_str(&compute_capability);

    let utilization_rates = match gpustat.utilization_rates {
        Ok(utilization_rates) => format!("{:>3} % | ", utilization_rates.gpu),
        Err(_err) => "".to_string(),
    };
    result.push_str(&utilization_rates);

    let memory_info = match gpustat.memory_info {
        Ok(memory_info) => format!("{:>width$} / {:<width$} MB | ", memory_info.used / 1024 / 1024, memory_info.total / 1024 / 1024,
            width = (memory_info.total / 1024 / 1024).to_string().chars().count()),
        Err(_err) => "".to_string(),
    };
    result.push_str(&memory_info);

    // let fan_speed = match gpustat.fan_speed {
    //     Ok(fan_speed) => format!("{:>3} % | ", fan_speed),
    //     Err(_err) => "".to_string(),
    // };
    // result.push_str(&fan_speed);

    let temperature = match gpustat.temperature {
        Ok(temperature) => format!("{:>3}°C | ", temperature),
        Err(_err) => "".to_string(),
    };
    result.push_str(&temperature);

    let mut sys: sysinfo::System = System::new_all();

    let graphics_processes = match gpustat.running_graphics_processes {
        Ok(processes) => {
            let mut all_processes_info = String::new();
            for p in processes.iter() {
                let p_name = get_process_name(&sys, p.pid);
                let used_mem = match p.used_gpu_memory {
                    UsedGpuMemory::Used(used) => used,
                    _ => 0,
                };
                all_processes_info.push_str(&format!("{}: {}MB  ", p_name, used_mem / 1024 / 1024));
            }
            all_processes_info
        },
        Err(_err) => "".to_string(),
    };
    result.push_str(&graphics_processes);

    // println!("{}", result);

    let mut stdout = stdout();
    stdout.queue(cursor::SavePosition);
    stdout.write(result.as_bytes());
    stdout.queue(cursor::RestorePosition);
    stdout.flush();
    thread::sleep(time::Duration::from_millis(500));
    // stdout.queue(crossterm::terminal::Clear(crossterm::terminal::ClearType::UntilNewLine));

    print!("\r");
}

fn get_process_name(sys: &sysinfo::System, pid: u32)-> String {
    sys.get_process(pid as i32).unwrap().name().to_string()
}