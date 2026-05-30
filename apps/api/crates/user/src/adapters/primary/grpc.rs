use std::sync::Arc;

use api::grpc::user::user_service_server::UserService;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use prost_types::Timestamp;
use tonic::Response;

use crate::{models, ports::Handler};

pub struct GrpcAdapter {
    core: Arc<dyn Handler>,
}

impl GrpcAdapter {
    pub fn new(core: Arc<dyn Handler>) -> Self {
        Self { core }
    }
}

#[async_trait]
impl UserService for GrpcAdapter {
    async fn get_user_with_password_by_email(
        &self,
        request: tonic::Request<api::grpc::user::GetUserWithPasswordByEmailRequest>,
    ) -> Result<tonic::Response<api::grpc::user::GetUserWithPasswordByEmailResponse>, tonic::Status>
    {
        let now = std::time::Instant::now();
        let r = request.into_inner();
        let user = self
            .core
            .find_with_password_by_email(r.email.as_str())
            .await
            .map(|user| user.into());

        let elapsed = now.elapsed();
        println!("Core Elapsed: {:.2?}", elapsed);
        Ok(Response::new(
            api::grpc::user::GetUserWithPasswordByEmailResponse { user: user },
        ))
    }
}

impl From<models::UserWithPassword> for api::grpc::user::UserWithPassword {
    fn from(user: models::UserWithPassword) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            password: user.password,
            created_at: Some(to_timestamp(user.created_at)),
            updated_at: Some(to_timestamp(user.updated_at)),
        }
    }
}

fn to_timestamp(dt: DateTime<Utc>) -> Timestamp {
    Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.timestamp_subsec_nanos() as i32,
    }
}
