use std::env;
use std::process;
use structopt::StructOpt;

fn ragequit(err_msg: &str) -> ! {
  eprintln!("{}", err_msg);
  process::exit(1);
}

arg_enum! {
  #[derive(Debug)]
  pub enum Environment {
    Production,
    Staging,
    Development,
  }
}

#[derive(StructOpt, Debug)]
struct Args {
  /// Port used for proxying
  #[structopt(short = "p", long = "port", default_value = "3000")]
  port: String,
  /// Environment
  #[structopt(
    raw(possible_values = r#"&["development","staging","production"]"#, case_insensitive = "true"),
    short = "e",
    long = "environment",
    default_value = "development"
  )]
  environment: Environment,
  /// Domain name in use for the services (only one)
  #[structopt(short = "d", long = "domain")]
  domain: Option<String>,
  /// Subdomain of the proxy service (self)
  #[structopt(short = "s", long = "subdomain")]
  subdomain: Option<String>,
}

#[derive(Debug)]
pub struct Config {
  pub self_subdomain: String,
  pub domain: String,
  pub google_client_id: String,
  pub google_client_secret: String,
  pub environment: Environment,
  pub port: u16,
}

impl Config {
  pub fn new() -> Self {
    let args = Args::from_args();
    Config {
      self_subdomain: args
        .subdomain
        .unwrap_or_else(|| env::var("OX_SELF_SUBDOMAIN").unwrap_or_else(|_| String::from("ox"))),
      domain: args
        .domain
        .or_else(|| env::var("OX_DOMAIN").ok())
        .unwrap_or_else(|| {
          ragequit("Please provide a domain for your services (env var or command line option)")
        }),
      google_client_id: env::var("GOOGLE_CLIENT_ID")
        .expect("Missing the GOOGLE_CLIENT_ID environment variable."),
      google_client_secret: env::var("GOOGLE_CLIENT_SECRET")
        .expect("Missing the GOOGLE_CLIENT_ID environment variable."),
      environment: args.environment,
      port: args.port.parse().unwrap(),
    }
  }
}
