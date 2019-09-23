extern crate chrono;
extern crate config;
extern crate csv;
extern crate dirs;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
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

#[derive(Debug)]
pub enum TyrError {
    IoError(io::Error),
    ConfigError(ConfigError),
    CsvError(csv::Error),
    TyrError(String),
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

    let path = get_path()?;

    trace!("Open file: {}", path);
    let file_buffer = File::open(path);

    let file_buffer = match file_buffer {
        Ok(f) => f,
        Err(ref e) if e.kind() == ErrorKind::NotFound {} => {
            debug!("file not found, return empty vec");
            return Ok(Vec::new());
        }
        Err(e) => {
            debug!("Error while opening file.");
            return Err(TyrError::IoError(e));
        }
    };

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

    let path = get_path()?;
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

    let mut records = read_records()?;
    records.push(record);
    write_records(records)?;
    Ok(())
}

pub fn print_times() -> Result<(), TyrError> {
    trace!("print_times()\n");

    let records = read_records()?;

    let mut entries = HashMap::new();
    for record in records {
        let time = Utc::now().with_second(0).unwrap().with_nanosecond(0).unwrap();
        let mut duration = record.stop.unwrap_or(time) - record.start;
        if record.stop.is_none() {
            println!("Currently working on: \"{}\"\n", record.title);
        }
        if entries.contains_key(&record.title) {
            debug!("entry with this title exists, sum up duration.");
            duration = *entries.get_mut(&record.title).unwrap() + duration;
        }
        entries.insert(record.title, duration);
    }

    for (title, duration) in entries {
        println!("{:02}:{:02} {}",
                 duration.num_hours(),
                 duration.num_minutes() - duration.num_hours() * 60,
                 title, );
    }
    println!();
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
        Ok(mut records) => records.pop(),
        Err(_) => None,
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
    //TODO: check if stop_time is before last progress was started

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

pub fn pause_progress(ref start_time: DateTime<Utc>, ref stop_time: DateTime<Utc>) -> Result<(), TyrError> {
    trace!("stop_progress({})", stop_time);

    let last = read_records()?.pop();
    if last.is_none() {
        return Err(TyrError::TyrError("No Records available.".to_string()));
    }
    let last = last.unwrap();
    if last.stop.is_some() {
        return Err(TyrError::TyrError("You are not currently working on anything.".to_string()));
    }
    //TODO: check if start_time is before last progress was started

    stop_progress(*start_time)?;
    start_progress(*stop_time, last.title)?;
    Ok(())
}

fn get_path() -> Result<String, ConfigError> {
    trace!("get_path()");

    //TODO: Handle config more generic. Write config file if it doesn't exist.

    let mut settings = config::Config::default();

    let mut config_dir = dirs::config_dir().unwrap();
    config_dir.push("tyr_config");
    debug!("Open config file: {:?}", config_dir);

    settings.merge(config::File::from(config_dir))?;
    settings.get("path")
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
