use log::{LevelFilter, info};
use p3_api::{structs::ship::RawShip, p3_access_api::local_p3_access_api::LocalP3AccessApi};
use p3_api::read_ship;

#[test]
fn test_ships() {
    let _ = simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Trace)
        .env()
        .init();

    assert_eq!(std::mem::size_of::<RawShip>(), 0x180);

    let mut api = LocalP3AccessApi::new(12496).unwrap();
    let ship = read_ship(&mut api, 0xd9);
    info!("{:?}", ship);
}
