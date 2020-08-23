use crate::core::Sheet;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct NotesMD {
    pub sheets: Vec<Sheet>,
    pub receiver: Receiver<String>,
    sender: Sender<String>,
}

impl NotesMD {
    pub fn new() -> Self {
        let (sender, receiver) = channel::<String>();

        Self {
            sheets: Vec::<Sheet>::new(),
            receiver,
            sender,
        }
    }

    pub fn new_sheet(&mut self) {
        let sheet = Sheet::default();

        self.sheets.push(sheet);
    }

    pub fn write(&self, _: usize, text: String) -> Option<String> {
        // if let Some(sheet) = self.sheets.get_mut(index) {
        //   let sheet = sheet;

        //   sheet.write(text);

        //   let final_text = sheet.raw_value.clone();

        //   Some(final_text)
        // } else {
        //   None
        // }

        match self.sender.send(text.clone()) {
            // messages get here successfully
            // is required to update the view from
            // this side accordingly
            Ok(()) => {}
            Err(err) => {
                println!("Error Sending: {}", err.to_string());
            }
        }

        Some(text)
    }

    pub fn receive(&self) -> Option<String> {
        for msg in self.receiver.try_recv() {
            println!("{}", msg);
        }

        None
    }
}
