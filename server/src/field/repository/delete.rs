use tonic::Status;

use super::Repository;

impl Repository {
    pub async fn delete(&self, id: i32) -> Result<(), Status> {
        let rows_updated = sqlx::query!(
            r#"
              UPDATE fields
              SET deleted_at = now()
              WHERE id = $1
                AND deleted_at IS NULL
            "#,
            id
        ).execute(&self.pool).await;

        match rows_updated {
            Ok(rows_updated) => match rows_updated.rows_affected() {
                0 => Err(Status::not_found("Field not found")),
                _ => Ok(()),
            },
            Err(_) => Err(Status::internal("An internal error occurred")),

        }
    }
}
