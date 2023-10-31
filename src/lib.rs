pub mod services {
    tonic::include_proto!("ecdar_proto_buf");
}

#[cfg(feature = "client")]
pub type Client = services::ecdar_backend_client::EcdarBackendClient<tonic::transport::Channel>;
