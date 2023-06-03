use std::ops::Add;
use std::time::Duration;

use async_std::task;
use chrono::{DateTime, Days, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use futures::prelude::*;
use influxdb2::Client;
use influxdb2_derive::{FromDataPoint, WriteDataPoint};
use log::{debug, LevelFilter};
use num_traits::FromPrimitive;
use p3_api::data::enums::WareId;
use p3_api::strum::IntoEnumIterator;
use p3_api::{
    data::{enums::TownId, game_world::GameWorldPtr},
    p3_access_api::open_process_p3_access_api::OpenProcessP3AccessApi,
};

#[derive(Debug, Default, WriteDataPoint)]
#[measurement = "town_wares_raw"]
struct TownWaresMeasurementRaw {
    #[influxdb(tag)]
    town: String,
    #[influxdb(tag)]
    ware: String,
    #[influxdb(field)]
    value: u64,
    #[influxdb(timestamp)]
    time: i64,
}

#[derive(Debug, Default, WriteDataPoint, FromDataPoint)]
#[measurement = "town_wares"]
struct TownWaresMeasurement {
    #[influxdb(tag)]
    town: String,
    #[influxdb(tag)]
    ware: String,
    #[influxdb(field)]
    value: u64,
    #[influxdb(timestamp)]
    time: i64,
}

async fn poll_town_wares() -> Result<(), Box<dyn std::error::Error>> {
    let bucket = "P3";
    let client = get_client();
    let mut api = OpenProcessP3AccessApi::new(13032).unwrap();
    let game_world = GameWorldPtr::default();
    let d = NaiveDate::from_ymd_opt(0, 1, 1).unwrap();
    let t = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
    let dt = NaiveDateTime::new(d, t);
    let mut last_utc: DateTime<Utc> = DateTime::from_utc(dt, Utc);
    loop {
        let p3_time = game_world.get_game_time(&mut api).unwrap();
        let d = NaiveDate::from_ymd_opt((700 + p3_time.year).try_into().unwrap(), 1, 1).unwrap();
        let t = NaiveTime::from_hms_milli_opt(p3_time.hour_of_day, p3_time.minute_of_hour, 0, 0).unwrap();
        let dt = NaiveDateTime::new(d, t);
        let dt = dt.add(Days::new(p3_time.day_of_year.into()));
        let utc: DateTime<Utc> = DateTime::from_utc(dt, Utc);
        if (utc - last_utc).num_days() == 0 {
            task::sleep(Duration::from_millis(200)).await;
            continue;
        }
        last_utc = utc;

        let mut raw_data = vec![];
        let mut data = vec![];
        for town_id in TownId::iter() {
            let town_ptr = game_world.get_town(town_id, &mut api).unwrap();
            let town_data = town_ptr.get_town_data();
            let wares = town_data.get_town_wares(&mut api).unwrap();
            for (index, amount) in wares.iter().enumerate() {
                let ware_id: WareId = FromPrimitive::from_u32(index as u32).unwrap();
                raw_data.push(TownWaresMeasurementRaw {
                    town: format!("{:?}", town_id),
                    ware: format!("{:?}", ware_id),
                    value: *amount as u64,
                    time: utc.timestamp_nanos(),
                });
                data.push(TownWaresMeasurement {
                    town: format!("{:?}", town_id),
                    ware: format!("{:?}", ware_id),
                    value: (*amount / ware_id.get_scaling()) as u64,
                    time: utc.timestamp_nanos(),
                });
            }
        }

        debug!("Pushing {} ({})", utc, utc.timestamp_nanos());
        client.write(bucket, stream::iter(raw_data)).await?;
        client.write(bucket, stream::iter(data)).await?;
    }
}

fn get_client() -> Client {
    let host = "http://localhost:8086";
    let org = "P3Org";
    let token = std::env::var("INFLUXDB2_TOKEN").unwrap();
    Client::new(host, org, token)
}

#[async_std::main]
async fn main() {
    simple_logger::SimpleLogger::new().with_level(LevelFilter::Debug).env().init().unwrap();

    poll_town_wares().await.unwrap()
}
