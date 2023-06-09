# Test Result

## Introduction

Our goal is to ensure the Kafka example functions properly on the EKS cluster. However, we've encountered some problems while attempting to deploy the example on the EKS cluster. In order to investigate these issues, we are performing the following checks:

1. We're using Docker Compose to run the current example to confirm its correctness.
2. We're converting the Docker Compose YAML file into an EKS configuration file.

For this example, all services—including MySQL, Zookeeper, and Kafka—are using native containers (non-wasm). The binlog application is compiled into wasm format and we have two ways to execute it: by using Docker Compose or by directly using wasmedge-cli.

## Current Status

### Services - MySQL + ZooKeeper + Kafka

| Environment | Services         |
| ----------- | ---------------- |
| macOS       | X [^mac-service] |
| Ubuntu      | O                |

### Binlog - Connect to the Services on Ubuntu

| Environment | Binlog Wasm (Docker Compose) | Binlog Wasm (WasmEdge CLI) |
| ----------- | ---------------------------- | -------------------------- |
| macOS       | X [^mac-wasm]                | X [^mac-wasmedge-wasm]     |
| Ubuntu      | X [^ubuntu-wasm]             | O [^ubuntu-wasmedge-wasm]  |

### Kubernetes - Run Service and Binlog

| Environment               | Result                |
| ------------------------- | --------------------- |
| kind                      | X [^k8s-kind-wasm]    |
| EKS                       | X [^k8s-eks-wasm]     |
| EKS with Ubuntu container | X [^k8s-eks-wasm-cli] |

[^mac-service]: no match for platform in manifest: not found ![#](https://github.com/second-state/wasmedge-mysql-binlog-kafka/blob/add-k8s/note/images/mac-service.png?raw=true)
[^mac-wasm]: connect successfully, but no logs after running insert.wasm ![#](https://github.com/second-state/wasmedge-mysql-binlog-kafka/blob/add-k8s/note/images/mac-wasm.png?raw=true)
[^mac-wasmedge-wasm]: Stuck after connecting to MySQL ![#](https://github.com/second-state/wasmedge-mysql-binlog-kafka/blob/add-k8s/note/images/mac-wasmedge-wasm.png?raw=true)
[^ubuntu-wasm]: Stuck after connecting to MySQL ![#](https://github.com/second-state/wasmedge-mysql-binlog-kafka/blob/add-k8s/note/images/ubuntu-wasm.png?raw=true)
[^ubuntu-wasmedge-wasm]: successfully running wasm ![#](https://github.com/second-state/wasmedge-mysql-binlog-kafka/blob/add-k8s/note/images/ubuntu-wasmedge-wasm.png?raw=true)
[^k8s-kind-wasm]: Throw fail to resolve url error after connecting to MySQL ![#](https://github.com/second-state/wasmedge-mysql-binlog-kafka/blob/add-k8s/note/images/k8s-kind-wasm.png?raw=true)
[^k8s-eks-wasm]: Throw fail to resolve url error after connecting to MySQL ![#](https://github.com/second-state/wasmedge-mysql-binlog-kafka/blob/add-k8s/note/images/k8s-eks-wasm.png?raw=true)
[^k8s-eks-wasm-cli]: Throw `thread 'main' panicked at 'mask too long'` error ![#](https://github.com/second-state/wasmedge-mysql-binlog-kafka/blob/add-k8s/note/images/k8s-eks-wasm-cli.png?raw=true)

## Environment

Here is a list of the software versions we're using for testing on macOS and Ubuntu:

- macOS

  ```bash
  $ uname -a
  Darwin GI-MBP.localdomain 22.4.0 Darwin Kernel Version 22.4.0: Mon Mar  6 20:59:58 PST 2023; root:xnu-8796.101.5~3/RELEASE_ARM64_T6020 arm64

  $ docker version
  Client:
   Cloud integration: v1.0.31
   Version:           23.0.5
   API version:       1.42
   Go version:        go1.19.8
   Git commit:        bc4487a
   Built:             Wed Apr 26 16:12:52 2023
   OS/Arch:           darwin/arm64
   Context:           default
  Server: Docker Desktop 4.19.0 (106363)
   Engine:
    Version:          dev
    API version:      1.43 (minimum version 1.12)
    Go version:       go1.20.3
    Git commit:       HEAD
    Built:            Tue Apr 25 09:07:47 2023
    OS/Arch:          linux/arm64
    Experimental:     false
   containerd:
    Version:          1.6.20
    GitCommit:        2806fc1057397dbaeefbea0e4e17bddfbd388f38
   runc:
    Version:          1.1.5
    GitCommit:        v1.1.5-0-gf19387a
   docker-init:
    Version:          0.19.0
    GitCommit:        de40ad0
  ```

- Ubuntu

  ```bash
  $ lsb_release -a
  No LSB modules are available.
  Distributor ID: Ubuntu
  Description: Ubuntu 22.04.2 LTS
  Release: 22.04
  Codename: jammy
  
  $ uname -a
  Linux vm-ubuntu 5.15.0-1039-azure #46-Ubuntu SMP Mon May 22 15:18:07 UTC 2023 x86_64 x86_64 x86_64 GNU/Linux
  
  $ docker version
  Client: Docker Engine - Community
   Cloud integration: v1.0.33
   Version:           24.0.2
   API version:       1.43
   Go version:        go1.20.4
   Git commit:        cb74dfc
   Built:             Thu May 25 21:51:00 2023
   OS/Arch:           linux/amd64
   Context:           desktop-linux
  
  Server: Docker Desktop 4.20.0 (109717)
   Engine:
    Version:          24.0.2
    API version:      1.43 (minimum version 1.12)
    Go version:       go1.20.4
    Git commit:       659604f9ee60f147020bdd444b26e4b5c636dc28
    Built:            Fri May 26 00:37:28 2023
    OS/Arch:          linux/amd64
    Experimental:     false
   containerd:
    Version:          1.6.21
    GitCommit:        3dce8eb055cbb6872793272b4f20ed16117344f8
   runc:
    Version:          1.1.7
    GitCommit:        v1.1.7-0-g860f061
   docker-init:
    Version:          0.19.0
    GitCommit:        de40ad0

  $ wasmedge --version
  wasmedge version 0.12.1
  ```

- kind

  ```bash
  $ kind version
  kind v0.18.0 go1.17.3 linux/amd64

  $ kubectl version --output=json
  {
    "clientVersion": {
      "major": "1",
      "minor": "26",
      "gitVersion": "v1.26.3",
      "gitCommit": "9e644106593f3f4aa98f8a84b23db5fa378900bd",
      "gitTreeState": "clean",
      "buildDate": "2023-03-15T13:40:17Z",
      "goVersion": "go1.19.7",
      "compiler": "gc",
      "platform": "linux/amd64"
    },
    "kustomizeVersion": "v4.5.7",
    "serverVersion": {
      "major": "1",
      "minor": "26",
      "gitVersion": "v1.26.3",
      "gitCommit": "9e644106593f3f4aa98f8a84b23db5fa378900bd",
      "gitTreeState": "clean",
      "buildDate": "2023-03-30T06:34:50Z",
      "goVersion": "go1.19.7",
      "compiler": "gc",
      "platform": "linux/amd64"
    }
  }
  ```

- EKS

  ```bash
  $ eksctl version
  0.141.0

  $ kubectl version --output=json
  {
    "clientVersion": {
      "major": "1",
      "minor": "26",
      "gitVersion": "v1.26.3",
      "gitCommit": "9e644106593f3f4aa98f8a84b23db5fa378900bd",
      "gitTreeState": "clean",
      "buildDate": "2023-03-15T13:40:17Z",
      "goVersion": "go1.19.7",
      "compiler": "gc",
      "platform": "linux/amd64"
    },
    "kustomizeVersion": "v4.5.7",
    "serverVersion": {
      "major": "1",
      "minor": "26+",
      "gitVersion": "v1.26.4-eks-0a21954",
      "gitCommit": "4a3479673cb6d9b63f1c69a67b57de30a4d9b781",
      "gitTreeState": "clean",
      "buildDate": "2023-04-15T00:33:09Z",
      "goVersion": "go1.19.8",
      "compiler": "gc",
      "platform": "linux/amd64"
    }
  }
  ```

## Components

We're testing the following components:

- Services
  - Use [docker-compose.yml](https://github.com/second-state/wasmedge-mysql-binlog-kafka/blob/04a110e/docker-compose.yml) to run kafka + zookeeper + mysql
- Binlog Wasm
  - (Docker Compose) Run docker image [secondstate/mysql-binlog-kafka](https://hub.docker.com/r/secondstate/mysql-binlog-kafka/tags).
  - (WasmEdge cli) Extract `/mysql-binlog-kafka.wasm` file from image [secondstate/mysql-binlog-kafka](https://hub.docker.com/r/secondstate/mysql-binlog-kafka/tags) and run it using WasmEdge cli.
  - Check if there are any logs coming from the wasm runtime after running `insert.wasm`.
- `insert.wasm`
  - After successfully running services and binlog, run [insert.wasm](https://github.com/second-state/wasmedge-mysql-binlog-kafka/blob/04a110e/mysql-binlog-kafka/sql-commands-test-wasm/insert.wasm) to insert data into MySQL.

## Steps

### Run Services

```bash
git clone https://github.com/second-state/wasmedge-mysql-binlog-kafka.git
cd wasmedge-mysql-binlog-kafka
docker compose -f docker-compose.yml up
```

### Run Binglog Wasm

#### Use Docker Compose

```bash
git clone https://github.com/second-state/wasmedge-mysql-binlog-kafka.git -b add-k8s
cd wasmedge-mysql-binlog-kafka/note
docker compose -f wasmedge.yml up
```

#### Use WasmEdge Cli

```bash
git clone https://github.com/second-state/wasmedge-mysql-binlog-kafka.git -b add-k8s
cd wasmedge-mysql-binlog-kafka/note
wasmedge --env "SLEEP_TIME=1000" --env "SQL_USERNAME=root" --env "SQL_PASSWORD=password" --env "SQL_PORT=3306" --env "SQL_HOSTNAME=localhost" --env "SQL_DATABASE=mysql" --env "KAFKA_URL=localhost:9092" mysql-binlog-kafka.wasm
```

#### Use kubernetes

Create the cluster:

```bash
# kind
kind create cluster

# EKS
eksctl create cluster -f note/eks-cluster.yml
```

Start services & binlog wasm:

```bash
git clone https://github.com/second-state/wasmedge-mysql-binlog-kafka.git -b add-k8s
cd wasmedge-mysql-binlog-kafka/note
helm repo add kwasm-ss https://second-state.github.io/kwasm-operator
helm install -n kwasm --create-namespace kwasm-operator kwasm-ss/kwasm-operator
kubectl annotate node --all kwasm.sh/kwasm-node=true
kubectl logs deployment.apps/kwasm-operator -n kwasm
helm install kafka oci://registry-1.docker.io/bitnamicharts/kafka
kubectl logs service/kafka
```

Run wasm using crun runtime:

```bash
kubectl apply -f kubernetes-binlog.yml
```

Run wasm using wasmedge cli in Ubuntu container (built from [note/Dockerfile.wasmedge](https://github.com/second-state/wasmedge-mysql-binlog-kafka/blob/add-k8s/note/Dockerfile.wasmedge)):

```bash
kubectl apply -f kubernetes-binlog-cli.yml
```

### Run insert.wasm

```bash
git clone https://github.com/second-state/wasmedge-mysql-binlog-kafka.git
cd wasmedge-mysql-binlog-kafka/mysql-binlog-kafka
wasmedge --env "DATABASE_URL=mysql://root:password@127.0.0.1:3306/mysql" sql-commands-test-wasm/insert.wasm
```

## Results

- Run services on macOS
  - Failed with error: `no match for platform in manifest: not found`.
- Run services on Ubuntu
  - Successfully run services.
- Run Binlog wasm (Docker Compose) on macOS
  - Successfully run Binlog wasm file, but not receive any logs from wasm runtime after executing `insert.wasm`.
- Run Binlog wasm (Docker Compose) on Ubuntu
  - Successfully run Binlog wasm file, but stuck after `Connected to mysql database`.
- Run Binlog wasm (WasmEdge cli) on macOS
  - Successfully run Binlog wasm file, but stuck after `Connected to mysql database`.
- Run Binlog wasm (WasmEdge cli) on Ubuntu
  - Successfully run Binlog wasm file, and receive logs from wasm runtime after executing `insert.wasm`.
- Run Binlog wasm (Kubernetes) with kind
  - Throw `Fail to resolve url` error after `Connected to mysql database`.
- Run Binlog wasm (Kubernetes) with EKS
  - Throw `Fail to resolve url` error after `Connected to mysql database`.
- Run Binlog wasm (Kubernetes) in Ubuntu container with EKS
  - Throw `thread 'main' panicked at 'mask too long'` error.

## Next Step

1. Figure out why the binlog wasm in the docker compose will hang.
2. Successfully run the binlog wasm in the kind environment locally.
3. Migrate the current configuration files to the EKS cluster.
