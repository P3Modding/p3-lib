use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

/// Patrician 3 navpoint matrix calculation
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Test,
    ConnectedNodesFromNavpointMatrix(ConnectedNodesFromNavpointMatrixArgs),
    ConnectedNodesFromNavigationData(ConnectedNodesFromNavigationDataArgs),
    NavpointMatrix(BuildNavpointMatrixArgs),
}

#[derive(Args, Debug)]
pub struct BuildNavpointMatrixArgs {
    /// Path to the connected nodes file
    #[arg(long = "connected-nodes", value_name = "connected-nodes-file")]
    pub connected_nodes_file: PathBuf,

    /// Path to the navigation vector file
    #[arg(long = "navigation-vector", value_name = "navigation-vector-file")]
    pub navigation_vector_file: PathBuf,

    /// Path to the output navpoint matrix file
    #[arg(long = "output", value_name = "output-file")]
    pub output_file: PathBuf,
}

#[derive(Args, Debug)]
pub struct ConnectedNodesFromNavpointMatrixArgs {
    /// Path to the input navpoint matrix file
    #[arg(long = "input", value_name = "input-file")]
    pub navpoint_matrix_file: PathBuf,

    /// Path to the output connected nodes file
    #[arg(long = "output", value_name = "output-file")]
    pub output_file: PathBuf,
}

#[derive(Args, Debug)]
pub struct ConnectedNodesFromNavigationDataArgs {
    /// Path to the input navigation matrix file
    #[arg(long = "navigation-matrix", value_name = "navigation-matrix-file")]
    pub navigation_matrix_file: PathBuf,

    /// Path to the input navigation vector file
    #[arg(long = "navigation-vector", value_name = "navigation-vector-file")]
    pub navigation_vector_file: PathBuf,

    /// Path to the output connected nodes file
    #[arg(long = "output", value_name = "output-file")]
    pub output_file: PathBuf,
}
