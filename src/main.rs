use crate::common::*;

mod common;
mod config;
mod error;
mod names;

fn main() -> Result<(), Error> {
  Config::from_args().run()
}
