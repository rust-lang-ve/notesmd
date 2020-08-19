use crate::core::Sheet;

pub struct NotesMD {
  pub sheets: Vec<Sheet>,
}

impl NotesMD {
  pub fn new() -> Self {
    Self {
      sheets: Vec::<Sheet>::new(),
    }
  }

  pub fn new_sheet(&mut self) {
    let sheet = Sheet::default();

    self.sheets.push(sheet);
  }

  pub fn write(&self, index: usize, text: String) -> Option<String> {
    // if let Some(sheet) = self.sheets.get_mut(index) {
    //   let sheet = sheet;
      
    //   sheet.write(text);
      
    //   let final_text = sheet.raw_value.clone();

    //   Some(final_text)
    // } else {
    //   None
    // }
    println!("{}", text);
    Some(text)
  }
}
