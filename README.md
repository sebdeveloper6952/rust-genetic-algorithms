### How to Run
1. install Docker
2. `docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD:/usr/src/myapp" -w /usr/src/myapp rust cargo build --release`
3. `docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD:/usr/src/myapp" -w /usr/src/myapp rust ./target/release/genetic-algorithms`