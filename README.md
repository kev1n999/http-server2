# HTTP Server

A simple HTTP server written in Rust for learning and study purposes.

## Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/kev1n999/http-server2
cd http-server2
```

### 2. Configure the Server Address

Edit `src/main.rs` and set the desired host and port:

```rust
const HOST: &str = "127.0.0.1:8000";
```

The default address is `127.0.0.1:8000`.

### 3. Build and Run the Server

```bash
cargo build
cargo run
```

Or simply:

```bash
cargo run
```

### 4. Open the Server in Your Browser

Navigate to:

```text
http://localhost:8000
```

## Registering Routes

Routes can be registered in `server/routes/routes.rs` by returning a `Vec<Route>`.

### Route Structure

```rust
struct Route {
    method: Method,      // HTTP method (GET, POST, PUT, PATCH, DELETE)
    path: String,        // Route path
    handler: Handler,    // Function responsible for handling the request
}
```

Create a route using:

```rust
Route::new(method, path, handler)
```

### Creating a Route Handler

Handler functions can be created inside `server/handler/`.

Example: responding to a `GET /` request.

```rust
pub fn home(ctx: &mut Context) -> Result<(), std::io::Error> {
    let home_file = parse_static_file("home.html")?;

    let response = Response::new(
        StatusCode::Ok,
        ContentType::Html,
        &home_file,
    );

    ctx.send(response)
}
```

In this example:

* `parse_static_file()` loads the file contents.
* `Response::new()` creates an HTTP response.
* `ctx.send()` sends the response to the client.

### Reading a JSON Request Body

The request body can be accessed through the request context:

```rust
pub fn response(ctx: &mut Context) -> Result<(), std::io::Error> {
    let Request { body, .. } = &ctx.request;
}
```

The body is stored as a `Vec<u8>`.

Example: handling a `POST /sum` request.

```rust
pub fn sum(ctx: &mut Context) -> Result<(), std::io::Error> {
    let Request { body, .. } = &ctx.request;

    #[derive(Deserialize)]
    struct Sum {
        x: i32,
        y: i32,
    }

    let json: Sum = parse_json(body)?;
    let sum = json.x + json.y;

    let response = Response::new(
        StatusCode::Ok,
        ContentType::Text,
        &sum.to_string(),
    );

    ctx.send(response)
}
```

### Registering Routes

The `routes()` function returns all available routes:

```rust
pub fn routes() -> Vec<Route> {
    vec![
        Route::new(Method::Get, "/".to_string(), home),
    ]
}
```

---

## Serving Static Files

Static files should be placed inside `server/public`.

Supported file types include:

* `.html`
* `.css`
* `.js`

### StaticFile Structure

```rust
struct StaticFile {
    path: String,
    content_type: ContentType,
}
```

Where:

* `path` is the public URL path (e.g. `/css/style.css`).
* `content_type` is the file MIME type.

### Registering Static Files

Static files are registered through the `static_files()` function:

```rust
pub fn static_files() -> Vec<StaticFile> {
    vec![
        StaticFile::new(
            "/js/calc.js".to_string(),
            ContentType::JavaScript,
        ),
        StaticFile::new(
            "/css/calc.css".to_string(),
            ContentType::Css,
        ),
    ]
}
```
