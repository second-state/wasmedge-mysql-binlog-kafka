## Run in EKS

### Create EKS cluster with Ubuntu nodes

- Select an AMI that matches your Kubernetes version at https://cloud-images.ubuntu.com/aws-eks/.
- For `eksctl` requirements, please choose `AmazonLinux2` as the AMI family.
- To utilize `containerd` as the container runtime, use the `overrideBootstrapCommand`.

```
eksctl create cluster -f eks.wasmedge-cluster.yml
```

### Setup kwasm

Since the default node name of EKS cluster is too long to use kwasm-operator. (see [this issue](https://github.com/KWasm/kwasm-operator/issues/21))
We forked kwasm-operator to use shorter job names at https://github.com/second-state/kwasm-operator/actions.

```
helm repo add kwasm-ss https://second-state.github.io/kwasm-operator
helm install -n kwasm --create-namespace kwasm-operator kwasm-ss/kwasm-operator
kubectl annotate node --all kwasm.sh/kwasm-node=true
```

### Run the kubernetes pod

Remember to change the image name and the env variables.

```
kubectl apply -f kubernetes.wasmedge-container.yml
```

## Run in kind

### Build & push the docker image

Remember to change the `DOCKER_HUB_USER` and the image name in `kubernetes.wasmedge-container.yml` file.

```
DOCKER_HUB_USER=dm4tw
git clone https://github.com/second-state/wasmedge-mysql-binlog-kafka.git
cd wasmedge-mysql-binlog-kafka
docker build -t $DOCKER_HUB_USER/wasmedge-mysql-binlog-kafka wasmedge-mysql-binlog-kafka
docker push $DOCKER_HUB_USER/wasmedge-mysql-binlog-kafka
```

### Setup the kind cluster

- Install [kind](https://kind.sigs.k8s.io) and [helm](https://helm.sh/docs/intro/install/).

```
kind create cluster
helm repo add kwasm http://kwasm.sh/kwasm-operator/
helm install -n kwasm --create-namespace kwasm-operator kwasm/kwasm-operator
kubectl annotate node --all kwasm.sh/kwasm-node=true
```

### Run the kubernetes pod

Remember to change the image name and the env variables.

```
kubectl apply -f kubernetes.wasmedge-container.yml
```
