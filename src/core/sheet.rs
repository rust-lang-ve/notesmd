use std::default::Default;

pub struct Sheet {
  pub raw_value: String,
}

impl Default for Sheet {
  fn default() -> Self {
    Self {
      raw_value: String::default(),
    }
  }
}

impl Sheet {
  pub fn write(&mut self, next_text: String) {
    println!("{}", next_text);
    self.raw_value = next_text;
  }
}
