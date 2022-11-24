//! Constants embedded into the implant at compile time
use rust_embed::RustEmbed;

pub const DOMAIN_NAME: &'static str = env!("MELTOUT_DOMAIN_NAME");
pub const ENDPOINT: &'static str = env!("MELTOUT_ENDPOINT");
// pub const CERT_FOLDER: &'static str = env!("MELTOUT_CERT_FOLDER");

#[derive(RustEmbed)]
#[folder = "$MELTOUT_CERT_FOLDER"]
#[include = "rootCA.pem"]
pub struct Certs;
