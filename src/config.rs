use crate::common::*;

#[derive(StructOpt)]
#[structopt(name = "unisearch", about = "Regex search for unicode character names")]
pub enum Config {
  #[structopt(name = "search")]
  Search {
    #[structopt(long = "ucd-names", name = "UCD-NAMES", parse(from_os_str))]
    ucd_names: PathBuf,

    #[structopt(name = "PATTERN", parse(try_from_str = "Config::parse_regex"))]
    regex: Regex,
  },
  #[structopt(name = "smear")]
  Smear {
    #[structopt(long = "ucd-names", name = "UCD-NAMES", parse(from_os_str))]
    ucd_names: PathBuf,

    #[structopt(long = "aesthetic")]
    aesthetic: bool,

    #[structopt(name = "TEMPLATE")]
    template: String,

    #[structopt(name = "COUNT", default_value = "0")]
    count: usize,
  },
}

impl Config {
  fn parse_regex(text: &str) -> Result<Regex, regex::Error> {
    RegexBuilder::new(text).case_insensitive(true).build()
  }

  pub fn run(self) -> Result<(), Error> {
    use Config::*;

    match self {
      Search { regex, ucd_names } => {
        let names = Names::from_ucd(ucd_names)?;

        let max_width = names
          .names
          .iter()
          .map(|(character, _)| UnicodeWidthChar::width(*character).unwrap_or(1))
          .max()
          .unwrap_or(1);

        for (character, name) in names.search(regex) {
          let codepoint: u32 = character.into();
          println!(
            "{:width$} - U+{:06X} - {}",
            character,
            codepoint,
            name,
            width = max_width,
          )
        }

        Ok(())
      }
      Smear {
        ucd_names,
        template,
        count,
        aesthetic,
      } => {
        let chars = template.chars().count();

        // initialize count vector to 1 for each character in template
        let mut counts = template.chars().map(|_| 1).collect::<Vec<usize>>();

        let mut rng = thread_rng();

        if count > chars {
          for _ in 0..(count - chars) {
            *counts.as_mut_slice().choose_mut(&mut rng).unwrap() += 1;
          }
        }

        let names = Names::from_ucd(ucd_names)?;

        let mut characters = Vec::new();

        for (character, count) in template.chars().zip(counts) {
          let pattern = format!(r"(?i)\blatin capital letter {}$", character);

          let regex = Regex::new(&pattern).unwrap();

          let matches = names
            .search(regex)
            .map(|(character, _name)| character)
            .collect::<Vec<char>>();

          for _ in 0..count {
            characters.push(*matches.choose(&mut rng).unwrap_or(&character));
          }
        }

        let strings = characters
          .iter()
          .map(char::to_string)
          .collect::<Vec<String>>();

        let output = if aesthetic {
          strings.join(" ")
        } else {
          strings.join("")
        };

        println!("{}", output);

        Ok(())
      }
    }
  }
}
