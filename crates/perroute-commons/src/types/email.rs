#[derive(sqlx::Type, Debug)]
#[sqlx(transparent)]
pub struct Email(String);

impl ToString for Email {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
