pub mod host {
    use const_format::concatcp;

    pub const STATE_DIR: &str = ".k3scontainer";
    pub const CONTAINER_NAME_PREFIX: &str = "k3scontainer-";
    pub const CLUSTER_NAME_FILE: &str = concatcp!(STATE_DIR,"/name");
    pub const CONTAINER_BUILD_FILE: &str = concatcp!(STATE_DIR,"/Dockerfile");
    pub const DATA_DIR: &str = concatcp!(STATE_DIR,"/data");
}

pub mod container {
    use const_format::concatcp;

    pub const ROOT_DIR: &str = "/k3scontainer";
    pub const DATA_DIR: &str = concatcp!(ROOT_DIR,"/data");
    pub const KEYS_DIR: &str = concatcp!(ROOT_DIR,"/keys");
    pub const INSTALLED_FILE: &str = concatcp!(ROOT_DIR,"/installed");
    pub const K3D_CLUSTER_CONFIG_FILE: &str = concatcp!(ROOT_DIR,"/cluster.k3d.yaml");
    pub const HOST_WORK_DIR_MOUNT: &str = concatcp!(ROOT_DIR,"/hwdm");
    pub const REPO_DIR: &str = concatcp!(ROOT_DIR,"/repo");
    pub const DOCKER_DIR: &str = "/var/lib/docker";
}
