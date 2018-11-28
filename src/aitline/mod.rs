use std::io;
use std::fs;
use std::path::{Path};
use std::str::FromStr;

enum LoadingErr {
    IOErr(io::Err),
    CSVErr(csv::Error),
}

pub struct MonthPassengerStat {
    month_name: String,
    passengers_count: i32
}

impl PassengersStatInfo for MonthPassengerStat{}

trait PassengerStat {
    fn get_month_name(&self) -> &str;
    fn get_passengers_count(&self) -> i32;
}

trait PassengersStatInfo:PassengerStat {
    fn info(&self) {
        println!("month_name:{}, passengers_count:{}", self.get_month_name(), self.get_passengers_count());
    }
}

impl MonthPassengerStat {
    pub fn from_csv_record (rec: &csv::StringRecord) -> Result<Self, <i32 as FromStr>::Err> {
        return MonthPassengerStat{
            month_name: rec[0].into(),
            passengers_count: i32::from_str(&rec[1])?
        };
    }
}

impl PassengerStat for MonthPassengerStat {
    fn get_month_name(&self) -> &str {
        return &self.month_name;
    }

    fn get_passengers_count(&self) -> i32 {
        return self.passengers_count;
    }
}

pub fn load_passengers<P: AsRef<Path>>(file_path: P) -> Result<Vec<MonthPassengerStat>, LoadingErr> {
    let file = fs::File::open(file_path)?;

    let mut buf_reader = io::BufReader::new(file);
    let mut rdr = csv::Reader::from_reader(buf_reader);

//    let mut res = Vec::new();

//    let hello = "some string";
//
//    let a = &hello[3..5];

//    for result in rdr.records() {
//        let stat = MonthPassengerStat::from_csv_record(&result?);
//
//        res.push(stat?);
//        println!("{:?}", record);
//    }

//    Ok(res)

    let res2 = rdr
        .records()
        .map(|rec|MonthPassengerStat::from_csv_record(&rec?)?)
        .collect::<Vec<_>>();

    Ok(res2)
}
