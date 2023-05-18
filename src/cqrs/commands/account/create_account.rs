use crate::{
    cqrs::message_bus::{Message, MessageHandler},
    crypto,
    database_models::{account::Account, user::User, user_password::UserPassword},
};
use async_trait::async_trait;
use error_stack::{Context, ResultExt};
use sqlx::PgPool;

#[derive(Debug)]
pub struct CreateAccountCommand {
    pub code: String,
    pub email: String,
    pub password: String,
}

impl Message for CreateAccountCommand {}

#[derive(Debug, thiserror::Error)]
pub enum CreateAccountError {
    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error("An account with code {0} already exists")]
    CodeAlreadyExists(String),

    #[error(transparent)]
    Crypto(#[from] argon2::password_hash::Error),
}

#[derive(Debug)]
pub struct CreateAccountHandler {
    pool: PgPool,
}

#[async_trait]
impl MessageHandler for CreateAccountHandler {
    type Message = CreateAccountCommand;

    type Output = Account;

    type Error = CreateAccountError;

    async fn handle(&self, message: Self::Message) -> Result<Self::Output, Self::Error> {
        if Account::find_by_code(&self.pool, &message.code)
            .await?
            .is_some()
        {
            return Err(CreateAccountError::CodeAlreadyExists(message.code.clone()));
        }

        let mut tx = self.pool.begin().await?;
        let account = Account::new(message.code).save(&mut tx).await?;
        let user = User::new(message.email, &account).save(&mut tx).await?;
        let _ = UserPassword::new(&user, crypto::hash_password(&message.password)?)
            .save(&mut tx)
            .await?;

        tx.commit().await?;

        Ok(account)
    }
}
