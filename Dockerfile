ARG K3D_VERSION=5.3.0-dind

FROM mikefarah/yq as yaml-query

FROM rancher/k3d:${K3D_VERSION} 

RUN apk add --no-cache bash rsync

COPY --from=yaml-query /usr/bin/yq /opt/yq/yq
RUN chmod +x /opt/yq/yq && ln -s /opt/yq/yq /usr/local/bin/yq

RUN curl -s https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash

RUN curl -s https://fluxcd.io/install.sh | sudo bash

COPY k3scontainer /k3scontainer/k3scontainer
RUN chmod +x /k3scontainer/k3scontainer
WORKDIR /k3scontainer
ENTRYPOINT ["./k3scontainer", "container", "entrypoint"]
