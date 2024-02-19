use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize)]
pub enum PolynomialTerm {
    Variable {
        symbol: String,
        power: u32,
        coefficient: u32,
    },
    Operator {
        value: String,
    },
    Constant {
        value: u32,
    },
}

impl Display for PolynomialTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PolynomialTerm::Variable {
                symbol,
                power,
                coefficient,
            } => write!(
                f,
                "{{polynomial_term_type: 'variable',\
      symbol: {},\
      power: {},\
      coefficient: {}}}",
                symbol, power, coefficient
            ),
            PolynomialTerm::Operator { value } => write!(
                f,
                "{{polynomial_term_type: 'operator',\
      value: {}}}",
                value
            ),
            PolynomialTerm::Constant { value } => write!(
                f,
                "{{polynomial_term_type: 'constant',\
      value: {}}}",
                value
            ),
            _ => write!(f, "display error: something went wrong"),
        }
    }
}

pub mod cli;
pub mod util;
pub mod proof;
