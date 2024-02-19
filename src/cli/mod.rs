use std::path::PathBuf;
use structopt::StructOpt;

/// Plonky2-proofs
#[derive(StructOpt, Debug)]
#[structopt(name = "Plonky2-proofs")]
struct Command {
    /// Input file
    #[allow(dead_code)]
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,
}

#[allow(dead_code)]
pub fn get_input_file_path() -> PathBuf {
  let command_opts = Command::from_args();
  command_opts.input
}