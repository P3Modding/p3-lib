use std::time::Duration;

use async_std::task;
use chrono::Utc;
use futures::prelude::*;
use influxdb2::Client;
use influxdb2_derive::WriteDataPoint;
use log::{debug, LevelFilter};
use num_traits::FromPrimitive;
use p3_api::data::enums::WareId;
use p3_api::strum::IntoEnumIterator;
use p3_api::{
    data::{enums::TownId, game_world::GameWorldPtr},
    p3_access_api::open_process_p3_access_api::OpenProcessP3AccessApi,
};

#[derive(Default, WriteDataPoint)]
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

#[derive(Default, WriteDataPoint)]
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
    let mut api = OpenProcessP3AccessApi::new(248).unwrap();
    let game_world = GameWorldPtr::default();
    loop {
        let mut raw_data = vec![];
        let mut data = vec![];
        for town_id in TownId::iter() {
            debug!("Polling Town {:?}", &town_id);
            let town_ptr = game_world.get_town(town_id, &mut api).unwrap();
            let town_data = town_ptr.get_town_data();
            let wares = town_data.get_town_wares(&mut api).unwrap();
            for (index, amount) in wares.iter().enumerate() {
                let ware_id: WareId = FromPrimitive::from_u32(index as u32).unwrap();
                raw_data.push(TownWaresMeasurementRaw {
                    town: format!("{:?}", town_id),
                    ware: format!("{:?}", ware_id),
                    value: *amount as u64,
                    time: Utc::now().timestamp_nanos(),
                });
                data.push(TownWaresMeasurement {
                    town: format!("{:?}", town_id),
                    ware: format!("{:?}", ware_id),
                    value: (*amount / ware_id.get_scaling()) as u64,
                    time: Utc::now().timestamp_nanos(),
                });
            }
        }

        let visby_beer = game_world.get_town(TownId::Visby, &mut api).unwrap().get_town_data().get_town_ware(WareId::Spices, &mut api).unwrap();
        debug!("Visby Beer = {} {}", visby_beer, visby_beer * WareId::Beer.get_scaling());
        client.write(bucket, stream::iter(raw_data)).await?;
        client.write(bucket, stream::iter(data)).await?;
        task::sleep(Duration::from_secs(5)).await;
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
    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .env()
        .init()
        .unwrap();
    poll_town_wares().await.unwrap()
}
