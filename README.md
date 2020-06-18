# Simple proxy

## Usage

```rust
use simple_proxy::middlewares::{router::*, Logger};
use simple_proxy::{Environment, SimpleProxy};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    port: u16,
}

#[derive(Debug, Clone)]
pub struct Config();

impl RouterConfig for Config {
    fn get_router_filename(&self) -> &'static str {
        "routes.json"
    }
}

#[tokio::main]
async fn main() {
    let args = Cli::from_args();

    let mut proxy = SimpleProxy::new(args.port, Environment::Development);
    let logger = Logger::new();
    let router = Router::new(&Config());

    // Order matters
    proxy.add_middleware(Box::new(router));
    proxy.add_middleware(Box::new(logger));

    // Start proxy
    let _ = proxy.run().await;
}
```

### Custom middleware

You can create your custom middleware by creating a struct implementing Middleware, consisting of 4 callbacks:

- `before_request` will be run every time
- `request_failure` will be run when the request fails
- `request_success` will be run when the request succeeds, you can then handle the response according to the status code or the body
- `after_request` will be run every time

#### For more info, see a [default middleware](src/middlewares/logger.rs)
