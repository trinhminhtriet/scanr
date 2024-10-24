# 🧹 ScanR

```text
 ____                       ____
/ ___|   ___   __ _  _ __  |  _ \
\___ \  / __| / _` || '_ \ | |_) |
 ___) || (__ | (_| || | | ||  _ <
|____/  \___| \__,_||_| |_||_| \_\

```

🧹 ScanR: A lightweight, fast, and configurable port scanner built in Rust for reliable multi-platform network scanning.

## 🚀 Installation

To install **scanr**, simply clone the repository and follow the instructions below:

```bash
git clone https://github.com/trinhminhtriet/scanr.git
cd scanr
cargo build --release
./target/release/scanr --version
```

Running the below command will globally install the `scanr` binary.

```bash
cargo install scanr
```

Optionally, you can add `~/.cargo/bin` to your PATH if it's not already there

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## 💡 Usage

Available command line arguments
| argument|result|
|--|--|
| `[string]` | The address or IP to scan. [default: `127.0.0.1`] |
|`-a` | Scan every port, from `1` to `65535`, conflicts with `-p` |
|`-c [number]` | How many concurrent request should be made. [default: `1000`] |
|`-m` | Monochrome mode - won't colourize the output [default: `false`] |
|`-p [number / string]` | Inclusive port range to scan, accepts either a range: `-300`, `101-200`, or a single port `80`, conflicts with `-a` [default: `-1000`] |
|`-r [number]` | Retry attempts per port. [default: `1`] |
|`-t [number]` | Timeout for each request in milliseconds. [default: `2000`] |
|`-6` | Scan the IPv6 address instead of IPv4, [default: `false`] |

### Examples

```shell
# Scan github.com using the default settings
scanr github.com

# Scan default address [127.0.0.1], all ports [1-65535],
# 2048 concurrent requests, 500ms timeout, 0 retries, IPv4
scanr -a -c 2048 -t 500 -r 0

# Scan www.google.com, ports 10-600,
# 500 concurrent requests, 3000ms timeout, default retries [1], IPv4
scanr www.google.com -p 10-600 -c 500 -t 3000

# Scan www.digitalocean.com, ports 1-100
# default concurrent requests[1000], 1000ms timeout, and use IPv6 address
scanr www.digitalocean.com -p -100 -t 1000 -6

# Scan www.bbc.com, port 443 only
# default concurrent requests[1000], default timeout[2000ms], 6 retries, IPv4
scanr www.bbc.com -p 443 -r 6
```

## 🗑️ Uninstallation

Running the below command will globally uninstall the `scanr` binary.

```bash
cargo uninstall scanr
```

Remove the project repo

```bash
rm -rf /path/to/git/clone/scanr
```

## 🤝 How to contribute

We welcome contributions!

- Fork this repository;
- Create a branch with your feature: `git checkout -b my-feature`;
- Commit your changes: `git commit -m "feat: my new feature"`;
- Push to your branch: `git push origin my-feature`.

Once your pull request has been merged, you can delete your branch.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
