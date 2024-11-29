use crate::data::enums::WareId;

#[derive(Debug)]
pub enum Operation {
    MoveShipToTown {
        ship_index: u32,
        town_index: u8,
    },
    ShipSellWares {
        amount: i32,
        ware_id: WareId,
        ship_index: u16,
        merchant_index: u16,
        town_index: u16,
    },
    ShipBuyWares {
        amount: i32,
        ware_id: WareId,
        ship_index: u16,
        merchant_index: u16,
        town_index: u16,
    },
    RepairShip {
        ship_index: u32,
    },
    ShipMoveWares {
        amount: i32,
        ship_index: u16,
        ware_id: WareId,
        merchant_index: u16,
        to_ship: bool,
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
    OfficeAutotradeSettingChange {
        stock: i32,
        price: i32,
        office_index: u32,
        ware_id: WareId,
    },
}

impl Operation {
    pub fn to_raw(&self) -> [u8; 0x14] {
        let mut op: [u8; 0x14] = [0; 0x14];
        match self {
            Operation::MoveShipToTown { ship_index, town_index } => {
                op[0x04..0x08].copy_from_slice(&ship_index.to_le_bytes());
                op[0x08..0x0c].copy_from_slice(&(*town_index as u32).to_le_bytes());
            }
            Operation::ShipSellWares {
                amount,
                ware_id,
                ship_index,
                merchant_index: field_8,
                town_index,
            } => {
                let opcode: u32 = 0x01;
                op[0x00..0x04].copy_from_slice(&opcode.to_le_bytes());
                op[0x04..0x08].copy_from_slice(&amount.to_le_bytes());
                op[0x08..0x0a].copy_from_slice(&(*ware_id as u16).to_le_bytes());
                op[0x0a..0x0c].copy_from_slice(&ship_index.to_le_bytes());
                op[0x0c..0x0e].copy_from_slice(&field_8.to_le_bytes());
                op[0x0e..0x10].copy_from_slice(&town_index.to_le_bytes());
            }
            Operation::ShipBuyWares {
                amount,
                ware_id,
                ship_index,
                merchant_index: field_8,
                town_index,
            } => {
                let opcode: u32 = 0x02;
                op[0x00..0x04].copy_from_slice(&opcode.to_le_bytes());
                op[0x04..0x08].copy_from_slice(&amount.to_le_bytes());
                op[0x08..0x0a].copy_from_slice(&(*ware_id as u16).to_le_bytes());
                op[0x0a..0x0c].copy_from_slice(&ship_index.to_le_bytes());
                op[0x0c..0x0e].copy_from_slice(&field_8.to_le_bytes());
                op[0x0e..0x10].copy_from_slice(&town_index.to_le_bytes());
            }
            Operation::RepairShip { ship_index: ship_id } => {
                let opcode: u32 = 0x03;
                op[0..4].copy_from_slice(&opcode.to_le_bytes());
                op[0x04..0x08].copy_from_slice(&ship_id.to_le_bytes());
            }
            Operation::ShipMoveWares {
                amount,
                ship_index,
                ware_id,
                merchant_index,
                to_ship,
            } => {
                let opcode: u32 = 0x08;
                let ware_id: u16 = *ware_id as _;
                op[0x00..0x04].copy_from_slice(&opcode.to_le_bytes());
                op[0x04..0x08].copy_from_slice(&amount.to_le_bytes());
                op[0x08..0x0a].copy_from_slice(&ship_index.to_le_bytes());
                op[0x0a..0x0c].copy_from_slice(&ware_id.to_le_bytes());
                op[0x0c..0x0e].copy_from_slice(&merchant_index.to_le_bytes());
                op[0x0e] = *to_ship as u8;
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
            Operation::OfficeAutotradeSettingChange {
                stock,
                price,
                office_index,
                ware_id,
            } => {
                let opcode: u32 = 0x5b;
                let ware_id: u32 = *ware_id as _;
                op[0..4].copy_from_slice(&opcode.to_le_bytes());
                op[4..8].copy_from_slice(&stock.to_le_bytes());
                op[8..0x0c].copy_from_slice(&price.to_le_bytes());
                op[0x0c..0x10].copy_from_slice(&office_index.to_le_bytes());
                op[0x10..0x14].copy_from_slice(&ware_id.to_le_bytes());
            }
        }
        op
    }
}
