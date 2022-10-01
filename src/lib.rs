pub mod ops;
pub mod cli;

use const_concat::const_concat;

const K3SCONTAINER_DIR: &str = ".k3scontainer";
const K3SCONTAINER_CLUSTER_NAME_FILE: &str = const_concat!("K3SCONTAINER_DIR","/name");
