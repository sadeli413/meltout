use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// The URL of the C2 team server, ie 'https://172.17.0.2:8001'
    #[arg(short, long, value_name="SERVER")]
    pub server: String,

    /// The root CA certificate of the C2 team server, ie 'certs/rootCA.pem'
    #[arg(short, long, value_name="PEM")]
    pub rootca: String,

    /// The domain name associated with the root CA pem file ie 'meltoutserver.c2'
    #[arg(short, long, value_name="DOMAIN")]
    pub name: String
}
