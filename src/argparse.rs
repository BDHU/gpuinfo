use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "gpu-info", about = "A list of command line flags.")]
pub struct Opt {
    /// Display all information
    #[structopt(short = "a", long = "all", about = "show all available GPU metrics.")]
    pub show_all: bool,

    #[structopt(short = "w", long = "watch", about = "watch over updated metrics.")]
    pub watch: bool,
}

pub fn argparse() -> Opt {
    let opt = Opt::from_args();
    return opt;
}