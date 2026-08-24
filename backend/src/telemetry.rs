use tracing_subscriber::{fmt, EnvFilter};

pub fn init_subscriber() {
 let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

 fmt().with_env_filter(env_filter).init();
}
