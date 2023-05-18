## Run in EKS

TBA

## Run in kind

### Build & push the docker image

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

```
kubectl apply -f kubernetes.wasmedge-container.yml
```
