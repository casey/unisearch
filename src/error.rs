use crate::common::*;

#[derive(Debug)]
pub enum Error {
  /// I/O error while reading data from UCD file
  Io { io_error: io::Error },
  /// Missing `;` separator in UCD record
  MissingSemicolon { line: String },
  /// Malformed hex in codepoint field
  Hex {
    text: String,
    parse_int_error: ParseIntError,
  },
  /// Out of range codepoint
  Codepoint { number: u32 },
  /// Missing `*` at the end of line with codepoint range
  MissingWildcard { line: String },
}

impl From<io::Error> for Error {
  fn from(io_error: io::Error) -> Error {
    Error::Io { io_error }
  }
}
