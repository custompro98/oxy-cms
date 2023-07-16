use sqlx::PgPool;
use tonic::{Request, Response, Status};

use super::pb::{CreateAccountRequest, Account, GetAccountRequest, UpdateAccountRequest, DeleteAccountRequest};
use super::pb::account_service_server::AccountService;

mod create;
mod get;
mod update;
mod delete;

pub struct Service {
    repository: super::repository::Repository,
}

impl Service {
    pub fn new(pool: PgPool) -> Service {
        Service {
            repository: super::repository::Repository::new(pool)
        }
    }
}

#[tonic::async_trait]
impl AccountService for Service {
    async fn create_account(
        &self,
        request: Request<CreateAccountRequest>,
    ) -> Result<Response<Account>, Status> {
        self.on_create_account(request).await
    }

    async fn get_account(
        &self,
        request: Request<GetAccountRequest>,
    ) -> Result<Response<Account>, Status> {
        self.on_get_account(request).await
    }

    async fn update_account(
        &self,
        request: Request<UpdateAccountRequest>,
    ) -> Result<Response<Account>, Status> {
        self.on_update_account(request).await
    }

    async fn delete_account(
        &self,
        request: Request<DeleteAccountRequest>,
    ) -> Result<Response<()>, Status> {
        self.on_delete_account(request).await
    }
}