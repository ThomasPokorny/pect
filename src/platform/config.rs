use std::env;
use std::net::SocketAddr;

// Import necessary modules
use dotenvy::dotenv;
use tokio::sync::OnceCell;

// Define a struct to represent server configuration
#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
}

// Define a struct that aggregates server and database configuration
#[derive(Debug)]
pub struct Config {
    server: ServerConfig,
}

// Implement methods for the Config struct to access configuration values
impl Config {
    // Getter method for the server host
    pub fn server_host(&self) -> &str {
        &self.server.host
    }

    // Getter method for the server port
    pub fn server_port(&self) -> u16 {
        self.server.port
    }
}

// Create a static OnceCell to store the application configuration
pub static CONFIG: OnceCell<Config> = OnceCell::const_new();

// Asynchronously initialize the configuration
async fn init_config() -> Config {
    // Load environment variables from a .env file if present
    dotenv().ok();

    // Create a ServerConfig instance with default values or values from environment variables
    let server_config = ServerConfig {
        host: env::var("HOST").unwrap_or_else(|_| String::from("127.0.0.1")),
        port: env::var("PORT")
            .unwrap_or_else(|_| String::from("3000"))
            .parse::<u16>()
            .unwrap(),
    };

    // Create a Config instance by combining server and database configurations
    Config {
        server: server_config,
    }
}

// Asynchronously retrieve the application configuration, initializing it if necessary
pub async fn config() -> &'static Config {
    // Get the configuration from the OnceCell or initialize it if it hasn't been set yet
    CONFIG.get_or_init(init_config).await
}

pub fn socket_address(config: &Config) -> SocketAddr {
    let address = format!("{}:{}", &config.server_host(), &config.server_port());
    address.parse().unwrap()
}
