# Test Result

## Summary

| Env          | Services         | Binlog Wasm (Docker Compose) | Binlog Wasm (WasmEdge cli) |
| ------------ | ---------------- | ---------------------------- | -------------------------- |
| macOS        | X [^mac-service] | X [^mac-wasm]                |                            |
| Ubuntu 20.04 | O                | X [^ubuntu-wasm]             | O [^wasmedge-wasm]         |
| EKS          | TBA              |                              |                            |

[^mac-service]: no match for platform in manifest: not found ![mac-service](https://github.com/second-state/wasmedge-mysql-binlog-kafka/blob/add-k8s/note/images/mac-service.png?raw=true)
[^mac-wasm]: connect successfully, but no logs after running insert.wasm ![mac-wasm](https://github.com/second-state/wasmedge-mysql-binlog-kafka/blob/add-k8s/note/images/mac-wasm.png?raw=true)
[^ubuntu-wasm]: operating system is not supported ![ubuntu-wasm](https://github.com/second-state/wasmedge-mysql-binlog-kafka/blob/add-k8s/note/images/ubuntu-wasm.png?raw=true)
[^wasmedge-wasm]: successfully running wasm ![wasmedge-wasm](https://github.com/second-state/wasmedge-mysql-binlog-kafka/blob/add-k8s/note/images/wasmedge-wasm.png?raw=true)

## Environment

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

- Ubuntu 20.04

  ```bash
  $ uname -a
  Linux neihu-2 5.15.0-52-generic #58~20.04.1-Ubuntu SMP Thu Oct 13 13:09:46 UTC 2022 x86_64 x86_64 x86_64 GNU/Linux
  $ docker verison
  Client: Docker Engine - Community
   Version:           23.0.4
   API version:       1.42
   Go version:        go1.19.8
   Git commit:        f480fb1
   Built:             Fri Apr 14 10:32:23 2023
   OS/Arch:           linux/amd64
   Context:           default

  Server: Docker Engine - Community
   Engine:
    Version:          23.0.4
    API version:      1.42 (minimum version 1.12)
    Go version:       go1.19.8
    Git commit:       cbce331
    Built:            Fri Apr 14 10:32:23 2023
    OS/Arch:          linux/amd64
    Experimental:     false
   containerd:
    Version:          1.6.21
    GitCommit:        3dce8eb055cbb6872793272b4f20ed16117344f8
   runc:
    Version:          1.1.7
    GitCommit:        v1.1.7-0-g860f061

  $ wasmedge --version
  wasmedge version 0.12.1
  ```

## Actions

- Services
  - Use [docker-compose.yml](https://github.com/second-state/wasmedge-mysql-binlog-kafka/blob/04a110e/docker-compose.yml) to run kafka + zookeeper + mysql
- `insert.wasm`
  - After successfully running services, run [insert.wasm](https://github.com/second-state/wasmedge-mysql-binlog-kafka/blob/04a110e/mysql-binlog-kafka/sql-commands-test-wasm/insert.wasm) to insert data into MySQL.
- Binlog Wasm
  - (Docker Compose) Run docker image [secondstate/mysql-binlog-kafka](https://hub.docker.com/r/secondstate/mysql-binlog-kafka/tags).
  - (WasmEdge cli) Extract `/mysql-binlog-kafka.wasm` file from image [secondstate/mysql-binlog-kafka](https://hub.docker.com/r/secondstate/mysql-binlog-kafka/tags) and run it using WasmEdge cli.
  - Check if there are any logs coming from the wasm runtime after running `insert.wasm`.

## Steps

### Run Services

```bash
git clone https://github.com/second-state/wasmedge-mysql-binlog-kafka.git
cd wasmedge-mysql-binlog-kafka
docker compose -f docker-compose.yml up
```

### Run insert.wasm

```bash
git clone https://github.com/second-state/wasmedge-mysql-binlog-kafka.git
cd wasmedge-mysql-binlog-kafka/mysql-binlog-kafka
wasmedge --env "DATABASE_URL=mysql://root:password@127.0.0.1:3306/mysql" sql-commands-test-wasm/insert.wasm
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
wasmedge --env "SLEEP_TIME=1000"  --env "SQL_USERNAM=root" --env "SQL_PASSWORD=password" --env "SQL_PORT=3306" --env "SQL_HOSTNAME=localhost" --env "SQL_DATABASE=mysql" --env "KAFKA_URL=localhost:9092" mysql-binlog-kafka.wasm
```

## Results

TBA

## Next Step

TBA
