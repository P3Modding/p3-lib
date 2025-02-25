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

    pub fn get_path_length2(&self, path: &[u16]) -> i32 {
        let mut distance = 0;
        let mut i = 0;
        while i < path.len() - 1 {
            let x1 = self.points[path[i] as usize].0 as i64;
            let y1 = self.points[path[i] as usize].1 as i64;
            let x2 = self.points[path[i + 1] as usize].0 as i64;
            let y2 = self.points[path[i + 1] as usize].1 as i64;
            let i_square = ((x2 << 16) - (x1 << 16)).pow(2) + ((y2 << 16) - (y1 << 16)).pow(2);
            distance += i_square.isqrt() as i32;
            i += 1;
        }

        distance
    }

    pub fn get_path_length3(&self, path: &[u16]) -> i32 {
        let mut distance = 0;
        let mut i = 0;
        while i < path.len() - 1 {
            let x1 = self.points[path[i] as usize].0 as i32;
            let y1 = self.points[path[i] as usize].1 as i32;
            let x2 = self.points[path[i + 1] as usize].0 as i32;
            let y2 = self.points[path[i + 1] as usize].1 as i32;
            let i_square = (x2 - x1).pow(2) + (y2 - y1).pow(2);
            distance += i_square.isqrt() << 16;
            i += 1;
        }

        distance
    }

    /// F64, multiplication and rounding at the end
    pub fn get_path_length_48k(&self, path: &[u16]) -> i32 {
        let mut distance = 0.0;
        let mut i = 0;
        while i < path.len() - 1 {
            let x1 = self.points[path[i] as usize].0 as i32;
            let y1 = self.points[path[i] as usize].1 as i32;
            let x2 = self.points[path[i + 1] as usize].0 as i32;
            let y2 = self.points[path[i + 1] as usize].1 as i32;
            let dx = (x2 - x1) as f64;
            let dy = (y2 - y1) as f64;
            distance += (dx * dx + dy * dy).sqrt();
            i += 1;
        }

        (distance * 65536.0).round() as i32
    }

    pub fn get_path_length_66k(&self, path: &[u16]) -> i32 {
        let mut distance = 0.0;
        let mut i = 0;
        while i < path.len() - 1 {
            let x1 = self.points[path[i] as usize].0 as i32;
            let y1 = self.points[path[i] as usize].1 as i32;
            let x2 = self.points[path[i + 1] as usize].0 as i32;
            let y2 = self.points[path[i + 1] as usize].1 as i32;
            let dx = (x2 - x1) as f32;
            let dy = (y2 - y1) as f32;
            distance += (dx * dx + dy * dy).sqrt();
            i += 1;
        }

        (distance * 65536.0).round() as i32
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
