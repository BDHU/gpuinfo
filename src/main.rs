mod argparse;
mod GPUstat;

use nvml_wrapper::NVML;
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::enum_wrappers::device::{Clock, TemperatureSensor};

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("Currently only Linux unsupported :(");
}

#[cfg(target_os = "linux")]
fn main() {
    let opt = argparse::argparse();

    let nvidia_gpu_stat = nvidia_gpu_exec(opt);
    let nvidia_gpu_stat = match nvidia_gpu_stat {
        Ok(nvidia_gpu_stat) => nvidia_gpu_stat,
        Err(error) => panic!("Failed to grab status for NVIDIA GPU, exiting with err: {:?}", error),
    };
    // TODO: amd_gpu_exec();
}

pub fn nvidia_gpu_exec(opt: argparse::Opt) -> Result<(), nvml_wrapper::error::NvmlErrorWithSource> {
    let nvml = NVML::init()?;
    let device_count = nvml.device_count()?;
    
    for i in 0..device_count {
        let device = nvml.device_by_index(i)?;
        print_device_info(device)?;
    }
    
    Ok(())
}

pub fn print_device_info(device: nvml_wrapper::Device) -> Result<(), nvml_wrapper::error::NvmlErrorWithSource> {
    let index = device.index()?;
    let name = device.name()?;
    let mem_info = device.memory_info()?;
    let temperature = device.temperature(TemperatureSensor::Gpu)?;


    print!("[{}]: {} | ", index, name);
    print!("{}°C | ", temperature);
    println!("{} / {} MB", mem_info.used / 1024 / 1024, mem_info.total / 1024 / 1024);

    Ok(())
}