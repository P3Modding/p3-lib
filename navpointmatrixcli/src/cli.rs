use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

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
    ShowPath(ShowPathArgs),
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

    /// F
    #[arg(long = "calculation-mode", value_name = "calculation-mode")]
    pub mode: ConnectedNodesMode,

    /// Path to the output connected nodes file
    #[arg(long = "output", value_name = "output-file")]
    pub output_file: PathBuf,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ConnectedNodesMode {
    BresenhamLine,
    //P3,
}

#[derive(Args, Debug)]
pub struct ShowPathArgs {
    /// Source node index
    #[arg(value_name = "source index", required = true)]
    pub source_index: u16,

    /// Destination node index
    #[arg(value_name = "destination index", required = true)]
    pub destination_index: u16,
}
