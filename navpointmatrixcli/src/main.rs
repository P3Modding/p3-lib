use clap::Parser;
use cli::{BuildNavpointMatrixArgs, Cli, Command, ConnectedNodesFromNavigationDataArgs, ConnectedNodesFromNavpointMatrixArgs};
use connected_nodes::ConnectedNodes;
use p3_api::data::{navigation_matrix::NavigationMatrix, navigation_vector::NavigationVector, navpoint_matrix::NavpointMatrix};
use pathfinding::prelude::{build_path, dijkstra_all};
use std::{
    fs::{self},
    time::Instant,
};

pub(crate) mod cli;
pub(crate) mod connected_nodes;
pub(crate) mod ordered_f32;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::NavpointMatrix(args) => build_navpoint_matrix(args),
        Command::ConnectedNodesFromNavpointMatrix(args) => build_connected_nodes_from_navpoint_matrix(args),
        Command::ConnectedNodesFromNavigationData(args) => build_connected_nodes_from_navigation_data(args),
        Command::Test => test(),
    }
}

fn build_navpoint_matrix(args: BuildNavpointMatrixArgs) {
    let navigation_vector = NavigationVector::deserialize(&fs::read(args.navigation_vector_file).unwrap());
    let connected_nodes = ConnectedNodes::deserialize(&fs::read(args.connected_nodes_file).unwrap());
    let mut new_navpoint_matrix = NavpointMatrix::new(navigation_vector.length);

    for (source_index, _) in navigation_vector.points.iter().enumerate() {
        let source_index = source_index as u16;
        let parents = dijkstra_all(&source_index, |n| connected_nodes.get_neighbours(*n, &navigation_vector));
        for target_index in 0..navigation_vector.points.len() as u16 {
            if source_index != target_index {
                let path = build_path(&(target_index), &parents);
                let distance = navigation_vector.get_path_length(&path);
                new_navpoint_matrix.set_next(source_index, target_index, path[1], distance, navigation_vector.length)
            } else {
                new_navpoint_matrix.set_next(source_index, source_index, source_index, 0, navigation_vector.length)
            }
        }
    }

    fs::write(args.output_file, new_navpoint_matrix.serialize()).unwrap();
}

fn build_connected_nodes_from_navpoint_matrix(args: ConnectedNodesFromNavpointMatrixArgs) {
    let input = fs::read(args.navpoint_matrix_file).unwrap();
    let connected_nodes = ConnectedNodes::from_navpoint_matrix(&NavpointMatrix::deserialize(&input));
    fs::write(args.output_file, connected_nodes.serialize()).unwrap();
}

fn build_connected_nodes_from_navigation_data(args: ConnectedNodesFromNavigationDataArgs) {
    let navigation_matrix = NavigationMatrix::deserialize(&fs::read(args.navigation_matrix_file).unwrap());
    let navigation_vector = NavigationVector::deserialize(&fs::read(args.navigation_vector_file).unwrap());
    let connected_nodes = ConnectedNodes::from_navigation_matrix(&navigation_vector, &navigation_matrix);
    fs::write(args.output_file, connected_nodes.serialize()).unwrap();
}

fn test() {
    let start = Instant::now();
    let navigation_vector = NavigationVector::deserialize(&fs::read(r"C:\Users\Benni\Patrician 3_workbench\navdata\nav_vec.dat").unwrap());
    let original_navpoint_matrix = NavpointMatrix::deserialize(&fs::read(r"C:\Users\Benni\Patrician 3_workbench\navdata\matrix_int.dat").unwrap());
    let connected_nodes = ConnectedNodes::from_navpoint_matrix(&original_navpoint_matrix);
    let mut new_navpoint_matrix = NavpointMatrix::new(navigation_vector.length);

    // Build new navpoint matrix
    for (source_index, _node) in navigation_vector.points.iter().enumerate() {
        let source_index = source_index as u16;
        let parents = dijkstra_all(&source_index, |n| connected_nodes.get_neighbours(*n, &navigation_vector));
        for target_index in 0..navigation_vector.points.len() as u16 {
            if source_index != target_index {
                let path = build_path(&(target_index), &parents);
                let distance = navigation_vector.get_path_length(&path);
                new_navpoint_matrix.set_next(source_index, target_index, path[1], distance, navigation_vector.length)
            } else {
                new_navpoint_matrix.set_next(source_index, source_index, source_index, 0, navigation_vector.length)
            }
        }
    }
    let duration = start.elapsed();
    println!("{duration:?}");

    // Assert equality
    assert_eq!(original_navpoint_matrix.matrix.len(), new_navpoint_matrix.matrix.len());
    println!("Asserting {} cells", original_navpoint_matrix.matrix.len());
    let mut bad_next_cells = 0;
    for i in 0..original_navpoint_matrix.matrix.len() {
        let orig_next = original_navpoint_matrix.matrix[i].next;
        let calculated_next = new_navpoint_matrix.matrix[i].next;
        if orig_next != calculated_next {
            println!("cell {i}: next {orig_next} != {calculated_next}");
            bad_next_cells += 1;
        }
    }
    println!("{bad_next_cells} bad next entries");

    let mut bad_distance_cells = 0;
    for i in 0..original_navpoint_matrix.matrix.len() {
        let orig_distance = original_navpoint_matrix.matrix[i].distance;
        let calculated_distance = new_navpoint_matrix.matrix[i].distance;
        if orig_distance != calculated_distance {
            println!("cell {i}: distance {orig_distance} != {calculated_distance}");
            bad_distance_cells += 1;
        }
    }
    println!("{bad_distance_cells} bad distances");
}
