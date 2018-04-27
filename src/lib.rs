extern crate chrono;
extern crate csv;
#[macro_use]
extern crate serde_derive;

use chrono::prelude::*;
use csv::Result;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    title: String,
    start: DateTime<Utc>,
    stop: Option<DateTime<Utc>>,
}

pub fn read_csv() -> Result<()> {
    let file_buffer = File::open("/tmp/tyr_test.csv")?;
    let mut reader = csv::Reader::from_reader(file_buffer);
    for result in reader.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

pub fn write_csv() -> Result<()> {
    let file_buffer = File::create("/tmp/tyr_test.csv")?;
    let mut wtr = csv::Writer::from_writer(file_buffer);

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
