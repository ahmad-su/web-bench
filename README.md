# web-bench
Web-Bench is a simple local web-server benchmark tool to test your server's speed in handling requests written in rust using rust's standard library

# Clone, and build on your system
Make sure you have Rust Language in your system

1. ```git clone https://github.com/ahmad-su/web-bench.git```
2. ```cd web-bench/```
3. ```cargo build --release``` (you can also install it on your system using ```cargo install --path .``` and run anywhere using ```web-bench [addr:port] [method] [path] [worker] [test_counts]```
4. ```cd target/release```
5. run ```./web-bench [addr:port] [method] [path] [worker] [test_counts]```

## Example run
Testing 127.0.0.1:8080 with request "GET / HTTP/1.1" for 100_000 counts @ 8 workers (threads):

```web-bench 127.0.0.1:8080 GET / 8 100000```

# Uninstalling
```cargo uninstall web-bench```