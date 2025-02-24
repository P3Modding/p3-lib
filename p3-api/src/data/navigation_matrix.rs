#[derive(Debug)]
pub struct NavigationMatrix {
    pub width: u16,
    pub height: u16,
    pub data: Vec<u8>,
}

impl NavigationMatrix {
    pub fn deserialize(data: &[u8]) -> Self {
        let width: u16 = u16::from_le_bytes(data[0..2].try_into().unwrap());
        let height: u16 = u16::from_le_bytes(data[2..4].try_into().unwrap());
        let data = data[4..].to_vec();
        NavigationMatrix { width, height, data }
    }
}
