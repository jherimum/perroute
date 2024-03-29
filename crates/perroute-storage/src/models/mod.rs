pub mod business_unit;
pub mod channel;
pub mod command_log;
pub mod connection;
pub mod db_event;
pub mod message;
pub mod message_dispatch;
pub mod message_type;
pub mod route;
pub mod schema;
pub mod template;

// pub struct Repository<E> {
//     executor: E,
// }

// pub trait WithExecutor<'e> {
//     type Executor: Executor<'e, Database = sqlx::postgres::Postgres>;

//     fn executor(&'e mut self) -> Self::Executor;
// }

// impl<'p> WithExecutor<'p> for Repository<sqlx::postgres::PgPool> {
//     type Executor = &'p sqlx::postgres::PgPool;

//     fn executor(&'p mut self) -> Self::Executor {
//         &self.executor
//     }
// }

// impl<'t, 'c> WithExecutor<'t> for Repository<sqlx::Transaction<'c, sqlx::Postgres>>
// where
//     'c: 't,
// {
//     type Executor = &'t mut sqlx::Transaction<'c, sqlx::Postgres>;

//     fn executor(&'t mut self) -> Self::Executor {
//         &mut self.executor
//     }
// }

// impl<'e, T> Repository<T>
// where
//     Self: WithExecutor<'e>,
// {
//     pub async fn submit(&'e mut self) {
//         Channel::new(&Code::from_str("CODE").unwrap(), "ss")
//             .save(self.executor())
//             .await
//             .unwrap();
//     }
// }
