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

    pub fn get_distance(&self, source_index: usize, destination_index: usize) -> f32 {
        let x1 = self.points[source_index].0 as f32;
        let y1 = self.points[source_index].1 as f32;
        let x2 = self.points[destination_index].0 as f32;
        let y2 = self.points[destination_index].1 as f32;
        let i_square = (x2 - x1).powf(2.0) + (y2 - y1).powf(2.0);
        i_square.sqrt()
    }
}
