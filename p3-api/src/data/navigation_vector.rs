#[derive(Debug)]
pub struct NavigationVector {
    pub length: u16,
    pub points: Vec<(i16, i16)>,
}

impl NavigationVector {
    pub fn deserialize(data: &[u8]) -> Self {
        let length: u16 = u16::from_le_bytes(data[0..2].try_into().unwrap());
        let mut points = vec![];
        for i in 0..length as usize {
            let x = i16::from_le_bytes(data[4 + 4 * i..4 + 4 * i + 2].try_into().unwrap());
            let y = i16::from_le_bytes(data[4 + 4 * i + 2..4 + 4 * i + 4].try_into().unwrap());
            points.push((x, y));
        }

        NavigationVector { length, points }
    }

    pub fn get_distance(&self, source_index: usize, destination_index: usize) -> i128 {
        let s = self.points[source_index];
        let d = self.points[destination_index];
        let i_square = (s.0 as i64 - d.0 as i64).pow(2) + (s.1 as i64 - d.1 as i64).pow(2);
        ((i_square as f64).sqrt()) as i128
    }
}
