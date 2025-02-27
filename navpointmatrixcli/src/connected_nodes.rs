use p3_api::data::{navigation_matrix::NavigationMatrix, navigation_vector::NavigationVector, navpoint_matrix::NavpointMatrix};

pub struct ConnectedNodes {
    pub connected_nodes: Vec<(u16, u16)>,
}

impl ConnectedNodes {
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = vec![];
        for pair in &self.connected_nodes {
            buf.extend_from_slice(&pair.0.to_le_bytes());
            buf.extend_from_slice(&pair.1.to_le_bytes());
        }
        buf
    }

    pub fn deserialize(data: &[u8]) -> Self {
        let len = data.len() / 4;
        let mut connected_nodes = Vec::with_capacity(len);
        for i in 0..len {
            let source = u16::from_le_bytes(data[i * 4..i * 4 + 2].try_into().unwrap());
            let destination = u16::from_le_bytes(data[i * 4 + 2..i * 4 + 4].try_into().unwrap());
            connected_nodes.push((source, destination));
        }

        Self { connected_nodes }
    }

    pub fn get_neighbours(&self, node_index: u16, nav_vec: &NavigationVector) -> Vec<(u16, i32)> {
        let mut neighbours = vec![];
        for n in &self.connected_nodes {
            if n.0 == node_index {
                neighbours.push((n.1, nav_vec.get_path_length(&[n.0 as _, n.1 as _])));
            }
        }

        neighbours
    }

    pub fn from_navigation_matrix(navigation_vector: &NavigationVector, navigation_matrix: &NavigationMatrix) -> Self {
        let mut nodes = vec![];
        for (source_index, source) in navigation_vector.points.iter().enumerate() {
            println!("Calculating neighbours for node {source_index}");
            for (destination_index, destination) in navigation_vector.points.iter().enumerate() {
                if is_connected(*source, *destination, navigation_matrix) {
                    nodes.push((source_index as _, destination_index as _))
                }
            }
        }

        ConnectedNodes { connected_nodes: nodes }
    }

    pub fn from_navpoint_matrix(navpoint_matrix: &NavpointMatrix) -> Self {
        let mut nodes = vec![];
        let nodes_count = navpoint_matrix.matrix.len().isqrt();
        for source_index in 0..nodes_count {
            for destination_index in 0..nodes_count {
                let cell = &navpoint_matrix.matrix[source_index * nodes_count + destination_index];
                if cell.next == destination_index as u16 {
                    nodes.push((source_index as _, destination_index as _));
                }
            }
        }
        ConnectedNodes { connected_nodes: nodes }
    }
}

fn is_connected(p0: (i16, i16), p1: (i16, i16), navigation_matrix: &NavigationMatrix) -> bool {
    if p0 == p1 {
        return true;
    }

    // Bresenham's Line Algorithm
    let (mut x0, mut y0) = p0;
    let (x1, y1) = p1;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut error = dx + dy;
    loop {
        if navigation_matrix.data[x0 as usize + navigation_matrix.width as usize * y0 as usize] == 1 {
            return false;
        }
        let e2 = 2 * error;
        if e2 >= dy {
            if x0 == x1 {
                break;
            }
            error += dy;
            x0 += sx;
        }
        if e2 <= dx {
            if y0 == y1 {
                break;
            }
            error += dx;
            y0 += sy;
        }
    }

    navigation_matrix.data[x0 as usize + navigation_matrix.width as usize * y0 as usize] == 0
}
