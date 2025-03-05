# Navpoint Matrix CLI

## Build Connected Nodes
- `cargo run --bin navpointmatrixcli -- connected-nodes-from-navpoint-matrix --input "C:\Users\Benni\Patrician 3_workbench\navdata\matrix_int.dat"  --output ./connected_nodes.navpointmatrix.dat`
- `cargo run --bin navpointmatrixcli -- connected-nodes-from-navigation-data --navigation-matrix "C:\Users\Benni\Patrician 3_workbench\navdata\nav_matrix.dat" --navigation-vector "C:\Users\Benni\Patrician 3_workbench\navdata\nav_vec.dat" --calculation-mode bresenham-line --output ./connected_nodes.navpointdata.dat`

## Build Navpoint Matrix
- `cargo run --release --bin navpointmatrixcli -- navpoint-matrix --navigation-vector "C:\Users\Benni\Patrician 3_workbench\navdata\nav_vec.dat" --connected-nodes .\connected_nodes.navpointmatrix.dat --output ./matrix_int.dat`
