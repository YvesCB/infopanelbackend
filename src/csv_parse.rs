use anyhow::Result;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use encoding_rs::Encoding;
use std::{fs, io::Read, path::PathBuf};

use crate::types::*;

#[derive(Debug)]
pub struct CSVParser {
    encoding: String,
    source_path: PathBuf,
    contents: Option<String>,
    parsed_events: Option<Vec<Event>>,
}

impl CSVParser {
    pub fn new(encoding: &str, path: PathBuf) -> CSVParser {
        CSVParser {
            encoding: encoding.to_string(),
            source_path: path.to_path_buf(),
            contents: None,
            parsed_events: None,
        }
    }

    #[allow(dead_code)]
    pub fn get_enc(&self) -> &str {
        &self.encoding
    }

    pub fn get_events(&self) -> Option<&Vec<Event>> {
        if let Some(events) = &self.parsed_events {
            return Some(&events);
        } else {
            return None;
        }
    }

    /// Read the file content and save to the `content` field.
    pub fn read_file(&mut self) -> Result<&Self> {
        let mut f = fs::File::open(&self.source_path)?;

        let mut file_content: Vec<u8> = Vec::with_capacity(100000);
        let _bytes_read = f.read_to_end(&mut file_content)?;

        if let Some(encoding) = Encoding::for_label(&self.encoding.as_bytes()) {
            let (result, _encoding, _errors) = encoding.decode(&file_content);

            // Remove '\r\n' when it's between "". This cleans up mistakes there were made when
            // titling
            let mut clean_result = String::new();
            let mut in_quotes = false;

            for c in result.chars() {
                match c {
                    '"' => {
                        in_quotes = !in_quotes;
                        clean_result.push(c);
                    }
                    '\r' if in_quotes => {}
                    '\n' if in_quotes => {}
                    _ => clean_result.push(c),
                }
            }
            self.contents = Some(clean_result);
        } else {
            return Err(anyhow::anyhow!("Invalid encoding specified."));
        }

        Ok(self)
    }

    /// Parse the file contents into the `Event` struct.
    pub fn parse_contents(&mut self) -> Result<()> {
        let mut events: Vec<Event> = Vec::with_capacity(1000);

        if let Some(read_content) = &self.contents {
            for (idx, line) in read_content.split("\r\n").enumerate() {
                // skip header line
                if idx > 0 && line.len() > 1 {
                    let clean_line = line.replace("\"", "");
                    let mut entries = clean_line.split(";");

                    let from_date = NaiveDate::parse_from_str(entries.next().unwrap(), "%d.%m.%Y")?;
                    let to_date = NaiveDate::parse_from_str(entries.next().unwrap(), "%d.%m.%Y")?;
                    let _weekday = entries.next();
                    let from_time = NaiveTime::parse_from_str(entries.next().unwrap(), "%H:%M")?;
                    let to_time = NaiveTime::parse_from_str(entries.next().unwrap(), "%H:%M")?;
                    let room = entries.next().unwrap().to_string();
                    let class_name = entries.next().unwrap().to_string();
                    let subject = entries.next().unwrap().to_string();
                    let teacher = entries.next().unwrap().to_string();
                    let department = entries.next().unwrap().to_string();
                    let building = entries.next().unwrap().to_string();
                    let event_id: u64 = entries.next().unwrap().parse()?;

                    events.push(Event {
                        event_id,
                        from_datetime: NaiveDateTime::new(from_date, from_time),
                        to_datetime: NaiveDateTime::new(to_date, to_time),
                        room,
                        class_name,
                        subject,
                        teacher,
                        department,
                        building,
                        visible: true,
                        modified_at: None,
                        modified_by: None,
                    })
                }
            }

            self.parsed_events = Some(events);
        } else {
            return Err(anyhow::anyhow!("No text loaded."));
        }

        Ok(())
    }
}
