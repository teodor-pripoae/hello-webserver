# Hello webserver

Hello world HTTP webservers in different languages

### Golang Net/Http

See [golang/](./golang)

```
$ wrk -t12 -c1400 -d20s http://127.0.0.1:8082/hello
Running 20s test @ http://127.0.0.1:8082/hello
  12 threads and 1400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.90ms    4.18ms  74.88ms   86.66%
    Req/Sec    37.80k    10.37k   88.19k    67.72%
  9021261 requests in 20.10s, 1.19GB read
  Socket errors: connect 385, read 0, write 0, timeout 0
Requests/sec: 448923.15
Transfer/sec:     60.79MB
```

### Ruby / Sinatra

See [ruby/](./ruby)

```
$ wrk -t12 -c1400 -d20s http://127.0.0.1:8083/hello
Running 20s test @ http://127.0.0.1:8083/hello
  12 threads and 1400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    12.92ms   71.16ms   1.99s    99.32%
    Req/Sec   682.08    482.66     4.47k    78.40%
  144933 requests in 20.04s, 27.40MB read
  Socket errors: connect 385, read 0, write 0, timeout 1620
Requests/sec:   7232.90
Transfer/sec:      1.37MB
```

### Kotlin / Quarkus

See [kotlin/](./kotlin)

```
$ wrk -t12 -c1400 -d20s http://127.0.0.1:8080/hello
Running 20s test @ http://127.0.0.1:8080/hello
  12 threads and 1400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     5.17ms    6.68ms 110.12ms   87.31%
    Req/Sec    19.44k     7.03k   46.23k    67.94%
  4649039 requests in 20.09s, 456.67MB read
  Socket errors: connect 385, read 0, write 0, timeout 0
Requests/sec: 231401.54
Transfer/sec:     22.73MB
```

### Rust / Dropshot

See [rust/](./rust)

```
$ wrk -t12 -c1400 -d20s http://127.0.0.1:8080/hello
Running 20s test @ http://127.0.0.1:8080/hello
  12 threads and 1400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     4.16ms   30.01ms 843.87ms   99.42%
    Req/Sec    35.76k    15.59k  136.78k    71.20%
  8416956 requests in 20.09s, 1.40GB read
  Socket errors: connect 385, read 0, write 0, timeout 0
Requests/sec: 418997.82
Transfer/sec:     71.13MB
```

### Crystal

See [crystal/](./crystal)

```
$ wrk -t12 -c1400 -d20s http://127.0.0.1:8080/hello
Running 20s test @ http://127.0.0.1:8080/hello
  12 threads and 1400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     4.93ms    1.04ms 210.03ms   94.87%
    Req/Sec    16.89k     6.29k   26.53k    42.88%
  4034355 requests in 20.04s, 438.61MB read
  Socket errors: connect 385, read 0, write 0, timeout 0
Requests/sec: 201338.70
Transfer/sec:     21.89MB
```

### Swift / Vapor

See [swift/](./swift)

```
$ wrk -t12 -c1400 -d20s http://127.0.0.1:8080/hello
Running 20s test @ http://127.0.0.1:8080/hello
  12 threads and 1400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    62.90ms   89.88ms   1.98s    98.79%
    Req/Sec     1.40k   635.59     6.78k    77.08%
  330918 requests in 20.04s, 52.07MB read
  Socket errors: connect 385, read 0, write 0, timeout 272
Requests/sec:  16512.96
Transfer/sec:      2.60MB
```
