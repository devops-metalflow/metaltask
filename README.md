# metaltask

[![Actions Status](https://github.com/devops-metalflow/metaltask/workflows/CI/badge.svg?branch=main&event=push)](https://github.com/devops-metalflow/metaltask/actions?query=workflow%3ACI)
[![License](https://img.shields.io/github/license/devops-metalflow/metaltask.svg?color=brightgreen)](https://github.com/devops-metalflow/metaltask/blob/main/LICENSE)
[![Tag](https://img.shields.io/github/tag/devops-metalflow/metaltask.svg?color=brightgreen)](https://github.com/devops-metalflow/metaltask/tags)
[![Gitter chat](https://badges.gitter.im/craftslab/devops-metalflow.png)](https://gitter.im/craftslab/devops-metalflow)



## Introduction

*metaltask* is a worker of *[metalflow](https://github.com/devops-metalflow/metalflow/)* written in Rust.



## Prerequisites

- Rust >= 1.59.0



## Run

```bash
./metaltask --config-file="config.yml" --listen-url="127.0.0.1:9090"
```



## Usage

```
USAGE:
    metaltask --config-file <NAME> --listen-url <URL>

OPTIONS:
    -c, --config-file <NAME>    Config file (.yml)
    -h, --help                  Print help information
    -l, --listen-url <URL>      Listen url (host:port)
    -V, --version               Print version information
```



## Settings

*metaltask* parameters can be set in the directory [config](https://github.com/devops-metalflow/metaltask/blob/main/src/config).

An example of configuration in [config.yml](https://github.com/devops-metalflow/metaltask/blob/main/src/config/config.yml):

```yaml
apiVersion: v1
kind: worker
metadata:
  name: metaltask
spec:
  task:
    clean: false
```



## Design

![design](design.png)



## License

Project License can be found [here](LICENSE).



## Reference

- [rust-grpc](https://gist.github.com/craftslab/c1b0e5c7f670d6f42a3623d04fddf8c1)
