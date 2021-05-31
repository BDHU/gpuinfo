use nvml_wrapper::{
    NVML,
};
use nvml_wrapper::enum_wrappers::device::{
    Clock,
    TemperatureSensor
};
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
    running_compute_processes: Result<Vec<ProcessInfo>, NvmlError>,
}

pub fn read_gpu_stat(device: nvml_wrapper::Device) -> GPUstat {
    let gpustat = GPUstat {
        name: device.name(),
        id: device.index(),
        compute_capability: device.cuda_compute_capability(),
        utilization_rates: device.utilization_rates(),
        memory_info: device.memory_info(),
        fan_speed: device.fan_speed(0), // Currently only take one fan, will add more fan readings
        temperature: device.temperature(TemperatureSensor::Gpu),
        running_compute_processes: device.running_compute_processes(),
    };

    return gpustat;
}

pub fn dump_gpu_stat(device: nvml_wrapper::Device) {
    let gpustat = read_gpu_stat(device);
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
        Ok(memory_info) => format!("{:>width1$} / {:<width2$} MB | ", memory_info.used / 1024 / 1024, memory_info.total / 1024 / 1024,
            width1 = (memory_info.total / 1024 / 1024).to_string().chars().count(),
            width2 = (memory_info.total / 1024 / 1024).to_string().chars().count()),
        Err(_err) => "".to_string(),
    };
    result.push_str(&memory_info);

    let fan_speed = match gpustat.fan_speed {
        Ok(fan_speed) => format!("{:>3} % | ", fan_speed),
        Err(_err) => "".to_string(),
    };
    result.push_str(&fan_speed);

    let temperature = match gpustat.temperature {
        Ok(temperature) => format!("{:>3}°C | ", temperature),
        Err(_err) => "".to_string(),
    };
    result.push_str(&temperature);

    println!("{}", result);
}