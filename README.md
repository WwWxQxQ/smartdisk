
## 0.Build
```shell
cargo build --release
sudo bash build.sh
```

## 1. Add Global PATH
in `~/.bashrc` or `~/.zshrc` add the following line:

```shell
#smartctl
SMARTCTL_HOME="/usr/local/bin"
export PATH=$SMARTCTL_HOME:$PATH
```

and then

```shell
source ~/.bashrc
````

## 2. Usage

```shell
Usage: smartdisk [OPTIONS] --device <DEVICE>

Options:
  -d, --device <DEVICE>  
  -f, --file <FILE>      [default: ./disk_record.md]
  -h, --help             Print help
  -V, --version          Print version
  ```