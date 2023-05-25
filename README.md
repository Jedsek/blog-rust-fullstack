# blog-rust-fullstack
This is a personal blog project written in rust fullstack, **still in early progress (I just create this project not long ago)**.  
- backend: actix-web
- frontend: yew
- database: sqlx(sqlite)

I will write a blog for writing this project, including most of the details.

Before you run this project, you should install `trunk`:  

```bash
cargo install trunk
```

- in `/client`:  

```bash
trunk serve
```

- in `/server`:  

```bash
cargo run
```

if you want hot-reload for backend server when file was changed, you could install `cargo-watch`:  

```bash
cargo install cargo-watch
```  

and then in `/server`:  

```bash
cargo watch -c -x run
```