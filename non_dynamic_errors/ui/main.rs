use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Debug)]
struct Thing;

impl Display for Thing {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("Thing")
    }
}

impl Error for Thing {}

fn main() {}
