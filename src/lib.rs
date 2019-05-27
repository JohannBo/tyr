extern crate chrono;
extern crate config;
extern crate csv;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io;
use std::io::ErrorKind;

use chrono::prelude::*;
use config::ConfigError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    title: String,
    start: DateTime<Utc>,
    stop: Option<DateTime<Utc>>,
}

fn read_records() -> Result<Vec<Record>, io::Error> {
    let path = get_path().unwrap();
    let file_buffer = File::open(path)?;
    let mut rd = csv::Reader::from_reader(file_buffer);

    let mut result = Vec::new();

    for record in rd.deserialize() {
        let record: Record = record?;
        result.push(record);
    }
    Ok(result)
}

fn write_records(records: Vec<Record>) -> Result<(), io::Error> {
    println!("write_records()");
    let path = get_path().unwrap();
    let file_buffer = File::create(&path)?;
    println!("create writer");
    let mut wtr = csv::Writer::from_writer(file_buffer);
    for record in records {
        wtr.serialize(record)?;
    }
    wtr.flush()?;
    Ok(())
}

fn append_record(record: Record) -> Result<(), io::Error> {
    let records = read_records();
    let mut records = match records {
        Ok(r) => r,
        Err(ref error) if error.kind() == ErrorKind::NotFound => Vec::new(),
        Err(e) => return Err(e),
    };
    records.push(record);
    write_records(records)?;
    Ok(())
}

pub fn print_records() -> Result<(), io::Error> {
    let records = read_records();
    let records = records?;
    println!("{:?}", records);
    Ok(())
}

pub fn write_csv() -> Result<(), io::Error> {

    let start = Utc.ymd(2018, 4, 27).and_hms(10, 50, 0);
    let stop = Utc::now();
    let stop = stop.with_second(0).unwrap();
    let stop = stop.with_nanosecond(0).unwrap();

    let record_1 = Record {
        title: "Ticket:1234,blah".to_string(),
        start,
        stop: None,
    };

    let record_2 = Record {
        title: "Ticket:1234".to_string(),
        start,
        stop: Some(stop),
    };
    append_record(record_1)?;
    append_record(record_2)?;

    Ok(())
}

fn get_path() -> Result<String, ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap();
    settings.get("path")
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
