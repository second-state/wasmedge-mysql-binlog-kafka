# Test Result

- Env
  - A: M2 Max macOS 13.3.1

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

  - B: Intel Ubuntu 20.04

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
    ```

  - C: Intel Ubuntu 20.04, run wasm file extracted from image [secondstate/mysql-binlog-kafka](https://hub.docker.com/r/secondstate/mysql-binlog-kafka/tags) with wasmedge cli

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

- Jobs to run:
  - Services: Use [docker-compose.yml](https://github.com/second-state/wasmedge-mysql-binlog-kafka/blob/04a110e/docker-compose.yml) to run kafka + zookeeper + mysql
  - Wasm: Run docker image [secondstate/mysql-binlog-kafka](https://hub.docker.com/r/secondstate/mysql-binlog-kafka/tags), and execute [insert.wasm](https://github.com/second-state/wasmedge-mysql-binlog-kafka/blob/04a110e/mysql-binlog-kafka/sql-commands-test-wasm/insert.wasm). After this, check if there are any logs coming from the wasm runtime.

## Steps

- Run insert.wasm (after successfully running services)

```bash
$ git clone https://github.com/second-state/wasmedge-mysql-binlog-kafka.git
$ cd wasmedge-mysql-binlog-kafka/mysql-binlog-kafka
$ wasmedge --env "DATABASE_URL=mysql://root:password@127.0.0.1:3306/mysql" sql-commands-test-wasm/insert.wasm
[src/bin/insert.rs:91] selected_payments = [
    Payment {
        customer_id: 1,
        amount: 2,
        account_name: None,
    },
    Payment {
        customer_id: 3,
        amount: 4,
        account_name: Some(
            "foo",
        ),
    },
    Payment {
        customer_id: 5,
        amount: 6,
        account_name: None,
    },
    Payment {
        customer_id: 7,
        amount: 8,
        account_name: None,
    },
    Payment {
        customer_id: 9,
        amount: 10,
        account_name: Some(
            "bar",
        ),
    },
]
Yay!
```

- Run services on Env A

```bash
$ git clone https://github.com/second-state/wasmedge-mysql-binlog-kafka.git
$ cd wasmedge-mysql-binlog-kafka
$ docker compose -f docker-compose.yml up
...
no match for platform in manifest: not found
```

- Run wasm on Env A

```bash
$ git clone https://github.com/second-state/wasmedge-mysql-binlog-kafka.git -b add-k8s
$ cd wasmedge-mysql-binlog-kafka/note
$ docker compose -f wasmedge.yml up
...
Attaching to wasmedge_binlog_kafka
wasmedge_binlog_kafka  | Thread started
wasmedge_binlog_kafka  | Connected to mysql database
wasmedge_binlog_kafka  | Connected to kafka server
wasmedge_binlog_kafka  | Topic already exist in Kafka
wasmedge_binlog_kafka  | Created kafka topic
```

- Run services on Env B

```bash
$ git clone https://github.com/second-state/wasmedge-mysql-binlog-kafka.git
$ cd wasmedge-mysql-binlog-kafka
$ docker compose -f docker-compose.yml up
[+] Building 0.0s (0/0)
[+] Running 3/3
Attaching to mysql5.7, wasmedge-mysql-binlog-kafka-kafka-1, wasmedge-mysql-binlog-kafka-zookeeper-1
mysql5.7                                 | 2023-05-30 10:02:40+00:00 [Note] [Entrypoint]: Entrypoint script for MySQL Server 5.7.42-1.el7 started.
mysql5.7                                 | 2023-05-30 10:02:41+00:00 [Note] [Entrypoint]: Switching to dedicated user 'mysql'
mysql5.7                                 | 2023-05-30 10:02:41+00:00 [Note] [Entrypoint]: Entrypoint script for MySQL Server
...
```

- Run wasm on Env B

```bash
$ git clone https://github.com/second-state/wasmedge-mysql-binlog-kafka.git -b add-k8s
$ cd wasmedge-mysql-binlog-kafka/note
$ docker compose -f wasmedge.yml up
...
image operating system "wasi" cannot be used on this platform: operating system is not supported
```

- Run wasm on env C

```bash
$ git clone https://github.com/second-state/wasmedge-mysql-binlog-kafka.git -b add-k8s
$ cd wasmedge-mysql-binlog-kafka/note
$ wasmedge --env "SLEEP_TIME=1000"  --env "SQL_USERNAM=root" --env "SQL_PASSWORD=password" --env "SQL_PORT=3306" --env "SQL_HOSTNAME=localhost" --env "SQL_DATABASE=mysql" --env "KAFKA_
URL=localhost:9092" mysql-binlog-kafka.wasm

Thread started
Connected to mysql database
Connected to kafka server
Created kafka topic
Got kafka partitionClient
Got kafka partition_offset
Received MySQL event
Try to create Kafka record
Kafka record created
Kafka record produced
Kafka new partition_offset
============================================== Event from Apache kafka ==========================================================================

Value: {"RotateEvent":{"binlog_filename":"mysql-bin.000004","binlog_position":194}}
Timestamp: 1970-01-01 00:00:00.042 UTC
Headers: {"timestamp":0,"event_type":4,"server_id":1,"event_length":47,"next_event_position":0,"event_flags":32}


Try to update MySQL replication
MySQL replication updated
Received MySQL event
Try to create Kafka record
Kafka record created
Kafka record produced
Kafka new partition_offset
============================================== Event from Apache kafka ======================================================
====================

Value: {"FormatDescriptionEvent":{"binlog_version":4,"server_version":"5.7.42-log","checksum_type":"Crc32"}}
Timestamp: 1970-01-01 00:00:00.042 UTC
Headers: {"timestamp":1685441813,"event_type":15,"server_id":1,"event_length":119,"next_event_position":0,"event_flags":0}


Try to update MySQL replication
MySQL replication updated

...
```

## Results

| Env | Services         | Wasm             |
| --- | ---------------- | ---------------- |
| A   | X [^mac-service] | X [^mac-wasm]    |
| B   | O                | X [^ubuntu-wasm] |
| C   |                  | O [^wasmedge-wasm]                 |

[^mac-service]: no match for platform in manifest: not found ![mac-service](https://github.com/second-state/wasmedge-mysql-binlog-kafka/blob/add-k8s/note/images/mac-service.png?raw=true)
[^mac-wasm]: connect successfully, but no logs after running insert.wasm ![mac-wasm](https://github.com/second-state/wasmedge-mysql-binlog-kafka/blob/add-k8s/note/images/mac-wasm.png?raw=true)
[^ubuntu-wasm]: operating system is not supported ![ubuntu-wasm](https://github.com/second-state/wasmedge-mysql-binlog-kafka/blob/add-k8s/note/images/ubuntu-wasm.png?raw=true)
[^wasmedge-wasm]: successfully running wasm ![wasmedge-wasm](https://github.com/second-state/wasmedge-mysql-binlog-kafka/blob/add-k8s/note/images/wasmedge-wasm.png?raw=true)
