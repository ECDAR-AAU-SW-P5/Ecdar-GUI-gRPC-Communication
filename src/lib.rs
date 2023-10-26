
pub mod services {
    tonic::include_proto!("ecdar_proto_buf");
}

pub use services::ecdar_backend_client::EcdarBackendClient as Client;
pub use services::ecdar_backend_server::EcdarBackendServer as Server;
pub use services::*;


