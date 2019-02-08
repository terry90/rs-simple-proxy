# Simple proxy

## Usage

```rust
extern crate simple_proxy;

mod middlewares;

use middlewares::auth::Auth;
use simple_proxy::middlewares::{Cors, Health, Logger, Router};
use simple_proxy::SimpleProxy;

fn main() {
    // Middlewares
    let auth = Auth::new(config.clone());
    let health = Health::new("/health", "OK !");
    let router = Router::new(config);
    let logger = Logger::new();
    let cors = Cors::new(
        "*",
        "GET, POST, PATCH, DELETE, OPTIONS",
        "Content-Type, Accept, Authorization, X-Requested-Ids, X-Tenant",
    );

    // Order matters
    proxy.add_middleware(Box::new(logger));
    proxy.add_middleware(Box::new(cors));
    proxy.add_middleware(Box::new(health));
    proxy.add_middleware(Box::new(router));
    proxy.add_middleware(Box::new(auth));

    // Start proxy
    proxy.run();
}

```

### Custom middleware

You can create your custom middleware by creating a struct implementing Middleware, consisting of 4 callbacks:

- `before_request` will be run every time
- `request_failure` will be run when the request fails
- `request_success` will be run when the request succeeds, you can then handle the response according to the status code or the body
- `after_request` will be run every time

#### For more info, see a [default middleware](src/middlewares/logger.rs)
