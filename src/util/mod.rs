// Handles file reading, JSON parsing

use thiserror::Error;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use super::PolynomialTerm;

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug)]
pub enum Errors {
    #[error("file open failed")]
    FileOpenError(#[source] io::Error),
    #[error("file read failed")]
    FileReadError(#[source] io::Error),
    #[error("wrong file extension")]
    FileExtensionError(),
    #[error("serialization of data failed")]
    SerializeError(#[from] serde_json::Error),
}

#[allow(dead_code)]
pub fn read_input_file(path: &PathBuf) -> Result<Vec<PolynomialTerm>, Errors> {
  use Errors::*;
  let mut file = File::open(path).map_err(FileOpenError)?;
  let file_ext = path.extension().unwrap();
  if file_ext.to_str().unwrap() != "json" {
    return Err(Errors::FileExtensionError());
  }
  let mut file_str = String::from("");
  file.read_to_string(&mut file_str).map_err(FileReadError)?;
  // println!("file contents: {}", file_str);
  let pt: Vec<PolynomialTerm> = serde_json::from_str(file_str.as_str()).map_err(SerializeError)?;
  // println!("poly term: {}", pt);
  Ok(pt)
}