use log::{info, LevelFilter};
use p3_api::data::enums::{TownId, WareId};
use p3_api::data::game_world::{GameWorldPtr, GAME_WORLD_ADDRESS};
use p3_api::read_ship;
use p3_api::{data::ship::RawShip, p3_access_api::open_process_p3_access_api::OpenProcessP3AccessApi};
use sysinfo::{PidExt, ProcessExt, System, SystemExt};

#[test]
fn test_ship_size() {
    assert_eq!(std::mem::size_of::<RawShip>(), 0x180);
}

#[test]
#[ignore]
fn test_ships() {
    let _ = simple_logger::SimpleLogger::new().with_level(LevelFilter::Trace).env().init();

    let s = System::new_all();
    for process in s.processes_by_name("Patrician") {
        let mut api = OpenProcessP3AccessApi::new(process.pid().as_u32()).unwrap();
        //let ship_id = 0xd9; // crayer 25
        let ship_id = 0x00;
        let ship = read_ship(&mut api, ship_id);
        info!("{:?}", ship);
    }
}

#[test]
#[ignore]
fn test_vispy_beer() {
    let _ = simple_logger::SimpleLogger::new().with_level(LevelFilter::Trace).env().init();

    let s = System::new_all();
    for process in s.processes_by_name("Patrician") {
        let mut api = OpenProcessP3AccessApi::new(process.pid().as_u32()).unwrap();
        let game_world = GameWorldPtr::default();
        let vispy = game_world.get_town(TownId::Visby, &mut api).unwrap();
        let beer = vispy.get_town_data().get_town_ware(WareId::Beer, &mut api).unwrap();
        info!("visby beer={}", beer);
    }
}
