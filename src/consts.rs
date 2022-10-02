pub mod host {
    use const_format::concatcp;

    const STATE_DIR: &str = ".k3scontainer";
    const CLUSTER_NAME_FILE: &str = concatcp!(STATE_DIR,"/name");
    const CONTAINER_BUILD_FILE: &str = concatcp!(STATE_DIR,"/Dockerfile");
    const DATA_DIR: &str = concatcp!(STATE_DIR,"/data");
}

pub mod container {
    use const_format::concatcp;

    const ROOT_DIR: &str = "/k3scontainer";
    const DATA_DIR: &str = concatcp!(ROOT_DIR,"/data");
    const KEYS_DIR: &str = concatcp!(ROOT_DIR,"/keys");
    const INSTALLED_FILE: &str = concatcp!(ROOT_DIR,"/installed");
    const K3D_CLUSTER_CONFIG_FILE: &str = concatcp!(ROOT_DIR,"/cluster.k3d.yaml");
    const HOST_WORK_DIR_MOUNT: &str = concatcp!(ROOT_DIR,"/hwdm");
    const REPO_DIR: &str = concatcp!(ROOT_DIR,"/repo");
    const DOCKER_DIR: &str = "/var/lib/docker";
}
