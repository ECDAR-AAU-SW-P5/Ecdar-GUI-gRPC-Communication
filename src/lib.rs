
pub mod services {
    tonic::include_proto!("ecdar_proto_buf");
}

use services::ecdar_backend_client::EcdarBackendClient;
pub type Client = EcdarBackendClient<Channel>;
use services::ecdar_backend_server::EcdarBackendServer;
pub type Server = EcdarBackendServer<Channel>;
pub use services::*;
use tonic::transport::Channel;


