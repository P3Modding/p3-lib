use std::ops::Add;
use std::time::Duration;

use async_std::task;
use chrono::{DateTime, Days, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use futures::prelude::*;
use influxdb2::Client;
use influxdb2_derive::{FromDataPoint, WriteDataPoint};
use log::{debug, error, LevelFilter};
use num_traits::FromPrimitive;
use p3_api::data::enums::{ShipWeaponId, WareId};
use p3_api::strum::IntoEnumIterator;
use p3_api::{
    data::{enums::TownId, game_world::GameWorldPtr},
    p3_access_api::open_process_p3_access_api::OpenProcessP3AccessApi,
};
use sysinfo::{PidExt, Process, ProcessExt, System, SystemExt};

#[derive(Debug, Default, WriteDataPoint)]
#[measurement = "storage_wares_raw"]
struct StorageMeasurementRaw {
    #[influxdb(tag)]
    storage_type: String,
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
#[measurement = "storage_wares"]
struct StorageMeasurement {
    #[influxdb(tag)]
    storage_type: String,
    #[influxdb(tag)]
    town: String,
    #[influxdb(tag)]
    ware: String,
    #[influxdb(field)]
    value: u64,
    #[influxdb(timestamp)]
    time: i64,
}

async fn poll_town_wares(pid: u32) -> Result<(), Box<dyn std::error::Error>> {
    let bucket = "P3";
    let client = get_client();
    let mut api = OpenProcessP3AccessApi::new(pid).unwrap();
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
            // Town data
            let town_storage = town_ptr.get_storage();
            let cutlasses = town_storage.get_cutlasses(&mut api).unwrap();
            data.push(StorageMeasurement {
                storage_type: "Town".to_string(),
                town: format!("{:?}", town_id),
                ware: "Cutlass".to_string(),
                value: cutlasses as u64,
                time: utc.timestamp_nanos(),
            });
            let wares = town_storage.get_wares(&mut api).unwrap();
            for (index, amount) in wares.iter().enumerate() {
                let ware_id: WareId = FromPrimitive::from_u32(index as u32).unwrap();
                raw_data.push(StorageMeasurementRaw {
                    storage_type: "Town".to_string(),
                    town: format!("{:?}", town_id),
                    ware: format!("{:?}", ware_id),
                    value: *amount as u64,
                    time: utc.timestamp_nanos(),
                });
                data.push(StorageMeasurement {
                    storage_type: "Town".to_string(),
                    town: format!("{:?}", town_id),
                    ware: format!("{:?}", ware_id),
                    value: (*amount / ware_id.get_scaling()) as u64,
                    time: utc.timestamp_nanos(),
                });
            }
            let weapons = town_storage.get_ship_weapons(&mut api).unwrap();
            for (index, amount) in weapons.iter().enumerate() {
                let ware_id: ShipWeaponId = FromPrimitive::from_u32(index as u32).unwrap();
                raw_data.push(StorageMeasurementRaw {
                    storage_type: "Town".to_string(),
                    town: format!("{:?}", town_id),
                    ware: format!("{:?}", ware_id),
                    value: *amount as u64,
                    time: utc.timestamp_nanos(),
                });
                data.push(StorageMeasurement {
                    storage_type: "Town".to_string(),
                    town: format!("{:?}", town_id),
                    ware: format!("{:?}", ware_id),
                    value: (*amount / ware_id.get_scaling()) as u64,
                    time: utc.timestamp_nanos(),
                });
            }

            // Office data
            // This is unsafe, because the office might be moved during execution (?)
            if let Some(office) = game_world.get_office_in_of(town_id, 0x24, &mut api).unwrap() {
                let office_storage = office.get_storage();
                let cutlasses = office_storage.get_cutlasses(&mut api).unwrap();
                data.push(StorageMeasurement {
                    storage_type: "Office".to_string(),
                    town: format!("{:?}", town_id),
                    ware: "Cutlass".to_string(),
                    value: cutlasses as u64,
                    time: utc.timestamp_nanos(),
                });
                let wares = office_storage.get_wares(&mut api).unwrap();
                for (index, amount) in wares.iter().enumerate() {
                    let ware_id: WareId = FromPrimitive::from_u32(index as u32).unwrap();
                    raw_data.push(StorageMeasurementRaw {
                        storage_type: "Office".to_string(),
                        town: format!("{:?}", town_id),
                        ware: format!("{:?}", ware_id),
                        value: *amount as u64,
                        time: utc.timestamp_nanos(),
                    });
                    data.push(StorageMeasurement {
                        storage_type: "Office".to_string(),
                        town: format!("{:?}", town_id),
                        ware: format!("{:?}", ware_id),
                        value: (*amount / ware_id.get_scaling()) as u64,
                        time: utc.timestamp_nanos(),
                    });
                }
                let weapons = office_storage.get_ship_weapons(&mut api).unwrap();
                for (index, amount) in weapons.iter().enumerate() {
                    let ware_id: ShipWeaponId = FromPrimitive::from_u32(index as u32).unwrap();
                    raw_data.push(StorageMeasurementRaw {
                        storage_type: "Office".to_string(),
                        town: format!("{:?}", town_id),
                        ware: format!("{:?}", ware_id),
                        value: *amount as u64,
                        time: utc.timestamp_nanos(),
                    });
                    data.push(StorageMeasurement {
                        storage_type: "Office".to_string(),
                        town: format!("{:?}", town_id),
                        ware: format!("{:?}", ware_id),
                        value: (*amount / ware_id.get_scaling()) as u64,
                        time: utc.timestamp_nanos(),
                    });
                }
            }
        }

        debug!("Pushing {} ({})", utc, utc.timestamp_nanos());
        debug!("{:?}", data);
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
    let s = System::new_all();
    let patricians: Vec<&Process> = s.processes_by_exact_name("Patrician3.exe").collect();
    if patricians.is_empty() {
        error!("Could not find Patrician3.exe");
        return;
    }

    poll_town_wares(patricians[0].pid().as_u32()).await.unwrap()
}
