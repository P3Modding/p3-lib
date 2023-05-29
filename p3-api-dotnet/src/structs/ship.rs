use p3_api::structs::ship::Ship;

#[repr(C)]
pub struct DotnetShip {
    pub merchant_id: u8,
    pub max_health: i32,
    pub current_health: i32,
    pub x: i32,
    pub y: i32,
    pub current_town_id: u8,
}

impl From<Ship> for DotnetShip {
    fn from(value: Ship) -> Self {
        Self {
            merchant_id: value.merchant_id,
            max_health: value.max_health,
            current_health: value.current_health,
            x: value.x,
            y: value.y,
            current_town_id: value.current_town_id,
        }
    }
}
