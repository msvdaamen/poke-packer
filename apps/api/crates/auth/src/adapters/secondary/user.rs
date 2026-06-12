use std::{str::FromStr, time::Instant};

use ::api::grpc::user::{
    GetUserWithPasswordByEmailRequest, user_service_client::UserServiceClient,
};
use async_trait::async_trait;
use tonic::transport::Channel;

use crate::{models::UserWithPassword, ports};
use shared::types::Email;

pub struct UserAdapter {
    channel: Channel,
}

impl UserAdapter {
    pub fn new(grpc_url: String) -> Self {
        let result = Channel::from_shared(grpc_url.clone());
        if let Ok(channel) = result {
            Self {
                channel: channel.connect_lazy(),
            }
        } else {
            panic!("Failed to connect to gRPC server");
        }
    }

    fn get_client(&self) -> UserServiceClient<Channel> {
        UserServiceClient::new(self.channel.clone())
    }
}

#[async_trait]
impl ports::User for UserAdapter {
    async fn find_with_password_by_email(
        &self,
        email: &Email,
    ) -> Result<Option<UserWithPassword>, Box<dyn std::error::Error>> {
        let now = Instant::now();
        let mut client = self.get_client();
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        let request = tonic::Request::new(GetUserWithPasswordByEmailRequest {
            email: email.as_str().to_string(),
        });
        let now = Instant::now();
        let response = client
            .get_user_with_password_by_email(request)
            .await?
            .into_inner();
        let user = response.user;
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        Ok(user.map(|u| UserWithPassword {
            id: uuid::Uuid::from_str(&u.id).unwrap(),
            password: u.password,
        }))
    }
}
