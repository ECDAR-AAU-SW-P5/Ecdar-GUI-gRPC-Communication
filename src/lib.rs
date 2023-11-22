pub mod services {
    tonic::include_proto!("ecdar_proto_buf");
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod test {
    #[ecdar_protobuff_rs_macros::serde_derive]
    enum ProtocOneOf {
        Json(String),
        Xml(String),
    }

    #[ecdar_protobuff_rs_macros::serde_derive]
    #[repr(i32)]
    enum ProtocEnum {
        T1 = 1,
    }
}
