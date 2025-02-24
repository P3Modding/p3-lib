use std::fs::{self};

use p3_api::data::{navigation_matrix::NavigationMatrix, navigation_vector::NavigationVector, navpoint_matrix::NavpointMatrix};
use pathfinding::prelude::{build_path, dijkstra_all};

pub struct DirectlyConnectedNodes {
    pub connected_nodes: Vec<(u16, u16)>,
}

fn main() {
    let _navigation_matrix = NavigationMatrix::deserialize(&fs::read(r"C:\Users\Benni\Patrician 3_workbench\navdata\nav_matrix.dat").unwrap());
    let navigation_vector = NavigationVector::deserialize(&fs::read(r"C:\Users\Benni\Patrician 3_workbench\navdata\nav_vec.dat").unwrap());
    let original_navpoint_matrix = NavpointMatrix::deserialize(&fs::read(r"C:\Users\Benni\Patrician 3_workbench\navdata\matrix_int.dat").unwrap());

    /*
    let original_connected_nodes = DirectlyConnectedNodes::from_navpoint_matrix(&original_navpoint_matrix);
    fs::write("./connections.orig.dat", original_connected_nodes.serialize()).unwrap();
    for (source, destination) in &original_connected_nodes.connected_nodes {
        println!("{source} {destination}")
    }
    */

    /*
    let calculated_connected_nodes = DirectlyConnectedNodes::from_navigation_vector(&navigation_vector, &navigation_matrix);
    fs::write("./connections.calculated.dat", calculated_connected_nodes.serialize()).unwrap();
    for (source, destination) in &calculated_connected_nodes.connected_nodes {
        println!("{source} {destination}")
    }
    */

    //let connected_nodes = DirectlyConnectedNodes::from_file(&fs::read("./connections.orig.dat").unwrap());

    let mut new_navpoint_matrix = NavpointMatrix::new(navigation_vector.length);
    let connected_nodes = DirectlyConnectedNodes::from_navpoint_matrix(&original_navpoint_matrix);
    for (source_index, _node) in navigation_vector.points.iter().enumerate() {
        let source_index = source_index as u16;
        let parents = dijkstra_all(&source_index, |n| connected_nodes.get_neighbours(*n, &navigation_vector));
        for target_index in 0..navigation_vector.points.len() as u16 {
            if source_index != target_index {
                let path = build_path(&(target_index), &parents);
                let distance = 0;
                //println!("{source_index} -> {target_index} {path:?}");
                new_navpoint_matrix.set_next(source_index, target_index, path[1], distance, navigation_vector.length)
            } else {
                new_navpoint_matrix.set_next(source_index, source_index, source_index, 0, navigation_vector.length)
            }
        }
    }

    {
        println!(
            "{} {} {}",
            navigation_vector.get_distance(0, 14),
            navigation_vector.get_distance(0, 9),
            navigation_vector.get_distance(9, 14)
        );
    }

    //assert_eq!(original_navpoint_matrix, new_navpoint_matrix)
    assert_eq!(original_navpoint_matrix.matrix.len(), new_navpoint_matrix.matrix.len());
    println!("Asserting {} cells", original_navpoint_matrix.matrix.len());
    //println!("{:?}", &original_navpoint_matrix.matrix[0..10]);
    //println!("{:?}", &new_navpoint_matrix.matrix[0..10]);
    for i in 0..original_navpoint_matrix.matrix.len() {
        let orig_next = original_navpoint_matrix.matrix[i].next;
        let calculated_next = new_navpoint_matrix.matrix[i].next;
        println!("cell {i}: {orig_next} == {calculated_next}");
        assert_eq!(orig_next, calculated_next);
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

impl DirectlyConnectedNodes {
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = vec![];
        for pair in &self.connected_nodes {
            buf.extend_from_slice(&pair.0.to_le_bytes());
            buf.extend_from_slice(&pair.1.to_le_bytes());
        }
        buf
    }

    pub fn get_neighbours(&self, node_index: u16, nav_vec: &NavigationVector) -> Vec<(u16, i128)> {
        let mut neighbours = vec![];
        for n in &self.connected_nodes {
            if n.0 == node_index {
                neighbours.push((n.1, nav_vec.get_distance(n.0 as _, n.1 as _)));
            }
        }

        neighbours
    }

    pub fn from_file(data: &[u8]) -> Self {
        let len = data.len() / 4;
        let mut connected_nodes = Vec::with_capacity(len);
        for i in 0..len {
            let source = u16::from_le_bytes(data[i..i + 2].try_into().unwrap());
            let destination = u16::from_le_bytes(data[i + 2..i + 4].try_into().unwrap());
            connected_nodes.push((source, destination));
        }

        Self { connected_nodes }
    }

    pub fn from_navigation_vector(navigation_vector: &NavigationVector, navigation_matrix: &NavigationMatrix) -> Self {
        let mut nodes = vec![];
        for (source_index, source) in navigation_vector.points.iter().enumerate() {
            println!("Calculating neighbours for node {source_index}");
            for (destination_index, destination) in navigation_vector.points.iter().enumerate() {
                if is_connected(*source, *destination, navigation_matrix) {
                    nodes.push((source_index as _, destination_index as _))
                }
            }
        }

        DirectlyConnectedNodes { connected_nodes: nodes }
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
        DirectlyConnectedNodes { connected_nodes: nodes }
    }
}
