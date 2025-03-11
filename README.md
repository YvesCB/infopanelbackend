# infopanelbackend
A backend for the Juventus Infopanel. It will be written in Rust.

## Frameworks
We will use [axum](https://docs.rs/axum/latest/axum/) to create the server. It will handle its own TLS encryption
eventually so that no HTTPS proxy service will be necessary. For that we will
reference the [tls-rustls example](https://github.com/tokio-rs/axum/tree/main/examples/tls-rustls).

For the database we will employ [SurrealDB](https://surrealdb.com) for its flexibility and deep
integration into Rust.
