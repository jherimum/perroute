pub mod configuration;
pub mod events;
pub mod postgres;
pub mod template;
pub mod types;

#[macro_export]
macro_rules! event {
    ($name:ident, { $( $field_name:ident : $field_type:ty ),* $(,)? }) => {
        paste::paste!{
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, bon::Builder)]
            pub struct [<$name>] {
                $( pub $field_name: $field_type ),*
            }

        }
    };
}
