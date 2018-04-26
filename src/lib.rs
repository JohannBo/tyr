extern crate csv;
#[macro_use]
extern crate serde_derive;

use csv::Result;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    title: String,
    start: String,
    stop: Option<String>,
}

pub fn write_csv() -> Result<()> {

    let file_buffer = File::create("/tmp/tyr_test.csv")?;
    let mut wtr = csv::Writer::from_writer(file_buffer);

    let record_1 = Record {
        title: "Ticket:1234,blah".to_string(),
        start: "12:00".to_string(),
        stop: None,
    };

    let record_2 = Record {
        title: "Ticket:1234".to_string(),
        start: "12:00".to_string(),
        stop: Some("18:00".to_string()),
    };

    wtr.serialize(&record_1)?;
    wtr.serialize(&record_2)?;

    wtr.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
