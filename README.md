# qdrant-rust
Rust App on Qdrant - Vector database

Note: Connecting to Qdrant cloud using Rust client, it uses only the gRPC interface so you must stick to the 6334 port.
When talking to the cloud you also make sure to use ```https``` in the URL when configuring the client.


## Build & Run 
```cargo build```

```cargo run```

### Qdrant Documenation:
[Qdrant Docs](https://qdrant.tech/documentation/)
