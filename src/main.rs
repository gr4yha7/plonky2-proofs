use anyhow::{Result, Error};
use plonky2_proofs::{cli, util};

// TODO build polynomial parser

fn main() -> Result<(), Error> {
    let input_file_path = cli::get_input_file_path();
    let res = util::read_input_file(&input_file_path);
    match res {
        Ok(poly_terms) => {
            for term in poly_terms.iter() {
                println!("poly term: {}", term)
            }
        },
        Err(e) => eprintln!("error: {}", e)
    }
    Ok(())
}
