mod argparse;
mod gpustat;

use gpustat::*;

use nvml_wrapper::NVML;

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
        dump_gpu_stat(device);
    }

    if opt.watch {
        loop {
            for i in 0..device_count {
                let device = nvml.device_by_index(i)?;
                dump_gpu_stat(device);
            }
        }
    }
    
    Ok(())
}
