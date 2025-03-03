use p3_api::data::{navigation_matrix::NavigationMatrix, navigation_vector::NavigationVector, navpoint_matrix::NavpointMatrix};

use crate::cli::ConnectedNodesMode;

#[derive(Debug, Clone)]
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

    pub fn from_navigation_matrix_p3(navigation_vector: &NavigationVector, navigation_matrix: &NavigationMatrix, mode: ConnectedNodesMode) -> Self {
        let mut nodes = vec![];
        for (source_index, source) in navigation_vector.points.iter().enumerate() {
            println!("Calculating neighbours for node {source_index}");
            for (destination_index, destination) in navigation_vector.points.iter().enumerate() {
                if is_connected(*source, *destination, navigation_matrix, mode) {
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

fn is_connected(p0: (i16, i16), p1: (i16, i16), navigation_matrix: &NavigationMatrix, mode: ConnectedNodesMode) -> bool {
    if p0 == p1 {
        return true;
    }

    let line = match mode {
        ConnectedNodesMode::BresenhamLine => get_bresenham_line(p0.0, p0.1, p1.0, p0.1),
        //ConnectedNodesMode::P3 => get_p3_line(p0.0, p0.1, p1.0, p0.1),
    };
    for point in line {
        if navigation_matrix.data[point.0 as usize + navigation_matrix.width as usize * point.1 as usize] == 1 {
            return false;
        }
    }
    true
}

pub fn get_bresenham_line(mut x0: i16, mut y0: i16, x1: i16, y1: i16) -> Vec<(i16, i16)> {
    let mut path = Vec::with_capacity(42);
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -((y1 - y0).abs());
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut error = dx + dy;
    loop {
        path.push((x0, y0));
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
    path
}

pub fn get_p3_line(mut x0: i16, mut y0: i16, x1: i16, y1: i16) -> Vec<(i16, i16)> {
    let mut path = Vec::with_capacity(42);
    path.push((x0, y0));
    let dx_abs = (x1 - x0).abs();
    let dy_abs = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut diff = dx_abs;
    let steps = dx_abs;
    for _ in 0..steps {
        // 0x0044477A
        x0 += sx; // Apply x step
        diff += 2 * dy_abs; // Add 2*dy to diff
        if diff >= 2 * dx_abs {
            // 0x00444784
            diff -= 2 * dx_abs;
            if diff == 0 {
                path.push((x0, y0));
                y0 += sy;
            } else {
                y0 += sy;
                path.push((x0, y0));
            }
        } else {
            // 0x00444793
            path.push((x0, y0));
        }
    }

    path
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{Path, PathBuf},
    };

    use regex::Regex;

    use super::*;

    #[test]
    fn it_works() {
        test_file(&PathBuf::from("tests/lines/line1.txt"));
        test_file(&PathBuf::from("tests/lines/line2.txt"));
        test_file(&PathBuf::from("tests/lines/line3.txt"));
    }

    fn test_file(path: &Path) {
        let data = fs::read_to_string(path).unwrap();
        let mut lines: Vec<&str> = data.split("\n").collect();
        let header = lines.remove(0);
        let header_re = Regex::new(r"^\((\d+)\, (\d+)\) -> \((\d+)\, (\d+)\)").unwrap();
        let line_re = Regex::new(r"^\((\d+)\, (\d+)\)").unwrap();
        let header_captures = header_re.captures(header).unwrap();
        let x0: i16 = header_captures.get(1).unwrap().as_str().parse().unwrap();
        let y0: i16 = header_captures.get(2).unwrap().as_str().parse().unwrap();
        let x1: i16 = header_captures.get(3).unwrap().as_str().parse().unwrap();
        let y1: i16 = header_captures.get(4).unwrap().as_str().parse().unwrap();

        let mut calculated_line = get_p3_line(x1, y1, x0, y0);
        calculated_line.remove(0);
        calculated_line.pop();
        println!("{calculated_line:?}");
        for (i, line) in lines.iter().enumerate() {
            let line_captures = line_re.captures(line).unwrap();
            let x: i16 = line_captures.get(1).unwrap().as_str().parse().unwrap();
            let y: i16 = line_captures.get(2).unwrap().as_str().parse().unwrap();
            println!("({}, {})", calculated_line[i].0, calculated_line[i].1);
            assert_eq!(x, calculated_line[i].0);
            assert_eq!(y, calculated_line[i].1);
        }
    }
}
