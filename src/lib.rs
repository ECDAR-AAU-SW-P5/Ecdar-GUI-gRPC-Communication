pub mod services {
    tonic::include_proto!("ecdar_proto_buf");
}

use services::ecdar_backend_client::EcdarBackendClient;

use tonic::transport::Channel;
pub type Client = EcdarBackendClient<Channel>;


