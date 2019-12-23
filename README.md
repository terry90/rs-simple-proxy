# Simple proxy

## Usage

```rust
use simple_proxy::middlewares::Logger;
use simple_proxy::{SimpleProxy, Environment};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    port: u16,
}

fn main() {
    let args = Cli::from_args();

    let mut proxy = SimpleProxy::new(args.port, Environment::Development);
    let logger = Logger::new();

    // Order matters
    proxy.add_middleware(Box::new(logger));

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
