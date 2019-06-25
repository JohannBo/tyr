extern crate chrono;
extern crate config;
extern crate csv;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io;
use std::io::ErrorKind;

use chrono::prelude::*;
use config::ConfigError;

use ::TyrError::IoError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    title: String,
    start: DateTime<Utc>,
    stop: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub enum TyrError {
    IoError(io::Error),
    ConfigError(ConfigError),
    CsvError(csv::Error),
}

impl From<io::Error> for TyrError {
    fn from(err: io::Error) -> TyrError {
        TyrError::IoError(err)
    }
}

impl From<ConfigError> for TyrError {
    fn from(err: ConfigError) -> TyrError {
        TyrError::ConfigError(err)
    }
}

impl From<csv::Error> for TyrError {
    fn from(err: csv::Error) -> Self {
        TyrError::CsvError(err)
    }
}


fn read_records() -> Result<Vec<Record>, TyrError> {
    trace!("read_records()");
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

fn write_records(records: Vec<Record>) -> Result<(), TyrError> {
    trace!("write_records({:?})", records);
    let path = get_path().unwrap();
    let file_buffer = File::create(&path)?;
    let mut wtr = csv::Writer::from_writer(file_buffer);
    for record in records {
        wtr.serialize(record)?;
    }
    wtr.flush()?;
    Ok(())
}

fn append_record(record: Record) -> Result<(), TyrError> {
    trace!("append_records({:?})", record);
    let records = read_records();
    let mut records = match records {
        Ok(r) => r,
        Err(my_error) => match my_error {
            IoError(ref e) if e.kind() == ErrorKind::NotFound => Vec::new(),
            _ => return Err(my_error)
        }
    };
    records.push(record);
    write_records(records)?;
    Ok(())
}

pub fn print_records() -> Result<(), TyrError> {
    trace!("print_records()");
    let records = read_records();
    let records = records?;
    for record in records {
        println!("{:?}", record)
    }
    Ok(())
}

pub fn write_demo_records() -> Result<(), TyrError> {
    trace!("write_demo_records()");

//    let start = Utc.ymd(2018, 4, 27).and_hms(10, 50, 0);
//    let stop = Utc::now();
//    let stop = stop.with_second(0).unwrap();
//    let stop = stop.with_nanosecond(0).unwrap();


    let start = Utc.ymd(2018, 4, 27).and_hms(10, 0, 0);
    let stop = Utc.ymd(2018, 4, 27).and_hms(11, 0, 0);
    let record_1 = Record {
        title: "Ticket-1234: do things".to_string(),
        start,
        stop: Some(stop),
    };

    let start = Utc.ymd(2018, 4, 27).and_hms(11, 0, 0);
    let stop = Utc.ymd(2018, 4, 27).and_hms(12, 0, 0);
    let record_2 = Record {
        title: "Ticket-4321: do other things".to_string(),
        start,
        stop: Some(stop),
    };

    let start = Utc.ymd(2018, 4, 27).and_hms(12, 0, 0);
    let stop = Utc.ymd(2018, 4, 27).and_hms(13, 0, 0);
    let record_3 = Record {
        title: "Ticket-1234: do things".to_string(),
        start,
        stop: Some(stop),
    };
    append_record(record_1)?;
    append_record(record_2)?;
    append_record(record_3)?;

    Ok(())
}

pub fn get_latest_record() -> Option<Record> {
    trace!("get_latest_record()");
    let records = read_records();
    match records {
        Ok(mut records) => {records.pop()},
        Err(_) => {None},
    }
}

pub fn start_progress(ref start: DateTime<Utc>, title: String) -> Result<(), TyrError> {
    trace!("start_progress({})", start);
    if get_latest_record().is_some() {
        stop_progress(*start)?;
    }

    let new_record = Record {
        title,
        start: *start,
        stop: None,
    };
    debug!("new record: {:?}", new_record);
    append_record(new_record)?;
    Ok(())
}

pub fn stop_progress(ref stop_time: DateTime<Utc>) -> Result<bool, TyrError> {
    trace!("stop_progress({})", stop_time);
    let mut records = read_records()?;
    let tail = match records.pop() {
        Some(x) => x,
        None => { return Ok(false); }
    };
    trace!("last record: {:?}", tail);
    if tail.stop.is_none() {
        let new_record = Record {
            title: tail.title,
            start: tail.start,
            stop: Some(*stop_time),
        };
        let mut new_record = vec![new_record];
        debug!("write stop time to record: {:?}", new_record);
        records.append(&mut new_record);
        write_records(records)?;
    }
    Ok(true)
}

fn get_path() -> Result<String, ConfigError> {
    trace!("get_path()");
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
