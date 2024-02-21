use super::enums::WareId;

#[derive(Debug)]
pub enum Operation {
    MoveShipToTown {
        ship_index: u32,
        town_index: u8,
    },
    RepairShip {
        ship_index: u32,
    },
    MoveWaresConvoy {
        raw_amount: i32,
        convoy_index: u16,
        ware: WareId,
        merchant_index: u16,
        to_ship: bool,
    },
    RepairConvoy {
        convoy_index: u32,
    },
}

impl Operation {
    pub fn to_raw(&self) -> [u8; 0x14] {
        let mut op: [u8; 0x14] = [0; 0x14];
        match self {
            Operation::MoveShipToTown {
                ship_index: ship_id,
                town_index: town,
            } => {
                let town_id: u16 = *town as _;
                op[0x04..0x08].copy_from_slice(&ship_id.to_le_bytes());
                op[0x08..0x0c].copy_from_slice(&(town_id as u32).to_le_bytes());
            }
            Operation::RepairShip { ship_index: ship_id } => {
                let opcode: u32 = 0x03;
                op[0..4].copy_from_slice(&opcode.to_le_bytes());
                op[0x04..0x08].copy_from_slice(&ship_id.to_le_bytes());
            }
            Operation::MoveWaresConvoy {
                raw_amount: amount,
                convoy_index: convoy_id,
                ware,
                merchant_index: merchant_id,
                to_ship,
            } => {
                let opcode: u32 = 0x1b;
                let ware_id: u16 = *ware as _;
                op[0..4].copy_from_slice(&opcode.to_le_bytes());
                op[4..8].copy_from_slice(&amount.to_le_bytes());
                op[8..0x0a].copy_from_slice(&convoy_id.to_le_bytes());
                op[0x0a..0x0c].copy_from_slice(&ware_id.to_le_bytes());
                op[0x0c..0x0e].copy_from_slice(&merchant_id.to_le_bytes());
                if *to_ship {
                    op[0x0e] = 1;
                } else {
                    op[0x0e] = 0;
                }
            }
            Operation::RepairConvoy { convoy_index: convoy_id } => {
                let opcode: u32 = 0x1d;
                op[0..4].copy_from_slice(&opcode.to_le_bytes());
                op[0x04..0x08].copy_from_slice(&convoy_id.to_le_bytes());
            }
        }
        op
    }
}
