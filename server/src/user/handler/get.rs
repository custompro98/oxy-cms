use tonic::{Request, Response, Status, Code};

use super::super::super::pb::user::{GetUserRequest, User};
use super::Handler;

impl Handler {
    pub async fn on_get_user(&self, request: Request<GetUserRequest>) -> Result<Response<User>, Status> {
        let user = self.repository.get(request.get_ref().id).await;

        match user {
            Ok(user) => Ok(Response::new(user)),
            Err(status) => match &status.code() {
                Code::NotFound => Err(status),
                Code::InvalidArgument => Err(status),
                Code::AlreadyExists => Err(status),
                Code::FailedPrecondition => Err(status),
                Code::PermissionDenied => Err(status),
                Code::Unauthenticated => Err(status),
                _ => Err(Status::internal("An internal error occurred")),
            },
        }
    }
}