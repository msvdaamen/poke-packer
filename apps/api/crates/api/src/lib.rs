pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("api_descriptor");

pub mod grpc {
    pub mod hello {
        tonic::include_proto!("grp.hello");
    }
    pub mod user {
        tonic::include_proto!("grp.user");
    }
}
