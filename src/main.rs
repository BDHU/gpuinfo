mod argparse;
mod gpustat;

use gpustat::*;

use nvml_wrapper::NVML;

use std::{thread, time::Duration};

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("Currently only Linux unsupported :(");
}

#[cfg(target_os = "linux")]
fn main() {
    let opt = argparse::argparse();

    // let nvidia_gpu_stat = nvidia_gpu_exec(opt);
    match nvidia_gpu_exec(opt) {
        Ok(_nvidia_gpu_stat) => (),
        Err(error) => panic!("Failed to grab status for NVIDIA GPU, exiting with err: {:?}", error),
    };
    // TODO: amd_gpu_exec();
}

pub fn nvidia_gpu_exec(opt: argparse::Opt) -> Result<(), nvml_wrapper::error::NvmlErrorWithSource> {
    let nvml = NVML::init()?;
    dump_all_gpu_stats(&nvml)?;

    if opt.watch {
        loop {
            dump_all_gpu_stats(&nvml)?;
            thread::sleep(Duration::from_secs(1));
        }
    }
    
    Ok(())
}
