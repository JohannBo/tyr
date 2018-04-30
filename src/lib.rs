extern crate chrono;
extern crate csv;
#[macro_use]
extern crate serde_derive;

use chrono::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    title: String,
    start: DateTime<Utc>,
    stop: Option<DateTime<Utc>>,
}

fn read_csv() -> Result<Vec<Record>, io::Error> {
    let file_buffer = File::open("/tmp/tyr_test.csv")?;
    let mut rd = csv::ReaderBuilder::new().has_headers(false).from_reader(file_buffer);

    let mut result = Vec::new();

    for record in rd.deserialize() {
        let record: Record = record?;
        result.push(record);
    }
    Ok(result)
}

pub fn print_records() -> Result<(), io::Error> {
    let records = read_csv();
    let records = records?;
    println!("{:?}", records);
    Ok(())
}

pub fn write_csv() -> Result<(), io::Error> {
    let path = "/tmp/tyr_test.csv";
    let file_buffer = OpenOptions::new().append(true).create(true).open(&path)?;

    let mut wtr = csv::WriterBuilder::new().has_headers(false).from_writer(file_buffer);

    let start = Utc.ymd(2018, 4, 27).and_hms(10, 50, 0);
    let stop = Utc::now();
    let stop = stop.with_second(0).unwrap();
    let stop = stop.with_nanosecond(0).unwrap();

    let record_1 = Record {
        title: "Ticket:1234,blah".to_string(),
        start: start,
        stop: None,
    };

    let record_2 = Record {
        title: "Ticket:1234".to_string(),
        start: start,
        stop: Some(stop),
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
