# web-bench
Web-Bench is a simple web-server benchmark tool to test your server's speed in handling requests written in rust using rust's standard library

# Clone, and build in your system
1. ```git clone https://github.com/ahmad-su/web-bench.git```
2. ```cd /web-bench```
3. ```cargo build --release```
4. ```cd target/release```
5. run ```.web-bench [addr:port] [method] [path] [worker] [test_counts]```

## Example run
Testing 127.0.0.1:8080 with "GET / HTTP/1.1\r\n\r\n" x 100_000 request @ 8 workers
./web-bench 127.0.0.1:8080 GET / 8 100000
