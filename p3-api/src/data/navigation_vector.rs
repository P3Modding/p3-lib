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

    pub fn get_path_length(&self, path: &[u16]) -> i32 {
        let mut distance = 0.0;
        for i in 0..path.len() - 1 {
            let x1 = self.points[path[i] as usize].0 as i32;
            let y1 = self.points[path[i] as usize].1 as i32;
            let x2 = self.points[path[i + 1] as usize].0 as i32;
            let y2 = self.points[path[i + 1] as usize].1 as i32;
            let dx = (x2 - x1) as f64;
            let dy = (y2 - y1) as f64;
            distance += (dx * dx + dy * dy).sqrt();
        }

        (distance * 65536.0).round() as i32
    }
}
