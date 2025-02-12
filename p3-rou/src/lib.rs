pub mod decompress;

pub struct TradeRouteFile {
    pub stops: Vec<TradeRouteStop>,
}

pub struct TradeRouteStop {
    pub town_index: u8,
    pub action: u8,
    pub order: [u8; 24],
    pub price: [i32; 24],
    pub amount: [i32; 24],
}

impl TradeRouteFile {
    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(4 + 220 * self.stops.len());
        let output_length = -(self.stops.len() as i32 * 220);
        data.extend_from_slice(&(output_length).to_le_bytes());
        for stop in &self.stops {
            stop.serialize(&mut data);
        }
        data
    }
}

impl TradeRouteStop {
    pub fn serialize(&self, output: &mut Vec<u8>) {
        output.extend_from_slice(&(0u16).to_le_bytes());
        output.extend_from_slice(&self.town_index.to_le_bytes());
        output.extend_from_slice(&self.action.to_le_bytes());
        output.extend_from_slice(&self.order);
        output.extend_from_slice(&convert(&self.price));
        output.extend_from_slice(&convert(&self.amount));
    }
}

// https://stackoverflow.com/a/72631195/1569755
pub fn convert(data: &[i32; 24]) -> [u8; 96] {
    let mut res = [0; 96];
    for i in 0..24 {
        res[4 * i..][..4].copy_from_slice(&data[i].to_le_bytes());
    }
    res
}
