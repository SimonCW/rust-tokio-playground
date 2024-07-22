Exploring async programming with tokio, axum, ...

Testing the axum server:

```bash
cargo run --bin axum-server
# in new shell
# test availability
curl http://127.0.0.1:3000/user/1
curl http://127.0.0.1:3000/user_optimized/1
# benchmark
ab -n 1000 -c 10 http://127.0.0.1:3000/user/1
ab -n 1000 -c 10 http://127.0.0.1:3000/user_optimized/1
```
