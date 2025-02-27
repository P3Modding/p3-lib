#[derive(Debug, Eq, PartialEq)]
pub struct NavpointMatrix {
    pub matrix: Vec<NavpointMatrixCell>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NavpointMatrixCell {
    pub distance: i32,
    pub next: u16,
}

impl Default for NavpointMatrixCell {
    fn default() -> Self {
        Self { distance: -1, next: u16::MAX }
    }
}

impl NavpointMatrix {
    pub fn new(width: u16) -> Self {
        Self {
            matrix: vec![Default::default(); (width as usize).pow(2)],
        }
    }

    pub fn deserialize(data: &[u8]) -> Self {
        let elements = data.len() / 6;
        let mut matrix = Vec::with_capacity(elements);
        for i in 0..elements {
            let distance = i32::from_le_bytes(data[i * 6..i * 6 + 4].try_into().unwrap());
            let next = u16::from_le_bytes(data[i * 6 + 4..i * 6 + 4 + 2].try_into().unwrap());
            matrix.push(NavpointMatrixCell { distance, next })
        }

        NavpointMatrix { matrix }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.matrix.len() * 6);
        for cell in &self.matrix {
            buf.extend_from_slice(&cell.distance.to_le_bytes());
            buf.extend_from_slice(&cell.next.to_le_bytes());
        }
        buf
    }

    pub fn set_next(&mut self, source: u16, destination: u16, next: u16, distance: i32, width: u16) {
        self.matrix[source as usize * width as usize + destination as usize] = NavpointMatrixCell { distance, next }
    }
}
