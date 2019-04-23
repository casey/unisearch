use crate::common::*;

pub struct Names {
  pub names: Vec<(char, String)>,
}

impl Names {
  pub fn from_ucd(path: impl AsRef<Path>) -> Result<Names, Error> {
    // convert to path
    let path = path.as_ref();

    // create empty names vec
    let mut names = Vec::new();

    // read file to string and loop over lines
    for line in fs::read_to_string(path)?.lines() {
      // drain (codepoint, name) tuples from line into names
      names.append(&mut Self::parse_line(line)?);
    }

    Ok(Names { names })
  }

  fn parse_line(line: &str) -> Result<Vec<(char, String)>, Error> {
    // skip comments
    if line.starts_with('#') {
      return Ok(Vec::new());
    }

    // trim whitespace
    let trimmed = line.trim();

    // skip blank lines
    if trimmed.is_empty() {
      return Ok(Vec::new());
    }

    // find field-delimiting `;` or return an error
    let semicolon_index = trimmed.find(';').ok_or_else(|| Error::MissingSemicolon {
      line: line.to_owned(),
    })?;

    // split at `;`
    let (codepoints, semicolon_name) = trimmed.split_at(semicolon_index);

    // trim whitespace from codepoint field
    let codepoints = codepoints.trim();

    // remove semicolon and trim whitespace from name field
    let name = semicolon_name[1..].trim();

    // codepoint fields are...
    match codepoints.find("..") {
      // ranges if they contain '..'
      Some(dotdot_index) => {
        let (start, dotdot_end) = codepoints.split_at(dotdot_index);
        let start: u32 = Self::parse_codepoint(start)?.into();
        let end: u32 = Self::parse_codepoint(&dotdot_end[2..])?.into();

        if !name.ends_with('*') {
          return Err(Error::MissingWildcard {
            line: line.to_owned(),
          });
        }

        let name_template = &name[..name.len() - 1];

        let mut names = Vec::new();

        for number in start..end {
          let character = Self::number_to_char(number)?;
          let name = format!("{}{:X}", name_template, number);
          names.push((character, name));
        }

        Ok(names)
      }
      // or single codepoints
      None => Ok(vec![(Self::parse_codepoint(codepoints)?, name.to_owned())]),
    }
  }

  fn parse_codepoint(text: &str) -> Result<char, Error> {
    let text = text.trim();

    let number = u32::from_str_radix(text, 16).map_err(|parse_int_error| Error::Hex {
      text: text.to_string(),
      parse_int_error,
    })?;

    Self::number_to_char(number)
  }

  fn number_to_char(number: u32) -> Result<char, Error> {
    char::from_u32(number).ok_or_else(|| Error::Codepoint { number })
  }

  pub fn search(&self, regex: Regex) -> impl Iterator<Item = (char, &str)> {
    self.names.iter().flat_map(move |(character, name)| {
      if regex.is_match(name) {
        Some((*character, name.as_str()))
      } else {
        None
      }
    })
  }
}
