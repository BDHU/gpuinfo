use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "gpu-info", about = "A list of command line flags.")]
pub struct Opt {
    /// Display all information
    #[structopt(short = "a", long = "all", about = "test.")]
    show_all: bool,
}

pub fn argparse() -> Opt {
    let opt = Opt::from_args();
    return opt;
}