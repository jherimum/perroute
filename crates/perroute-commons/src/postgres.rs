#[macro_export]
macro_rules! impl_sqlx_type {
    ( $in_ty:ty as $out_ty:ty) => {
        impl sqlx::Type<sqlx::Postgres> for $in_ty {
            fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
                <$out_ty as sqlx::Type<sqlx::Postgres>>::type_info()
            }

            fn compatible(
                ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo,
            ) -> bool {
                <$out_ty as sqlx::Type<sqlx::Postgres>>::compatible(ty)
            }
        }

        impl<'q> sqlx::Encode<'q, sqlx::Postgres> for $in_ty {
            fn encode_by_ref(
                &self,
                buf: &mut sqlx::postgres::PgArgumentBuffer,
            ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
                <$out_ty as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(
                    &self.to_string(),
                    buf,
                )
            }
        }

        impl<'r> sqlx::Decode<'r, sqlx::Postgres> for $in_ty {
            fn decode(
                value: sqlx::postgres::PgValueRef<'r>,
            ) -> Result<Self, sqlx::error::BoxDynError> {
                let value =
                    <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
                Ok(value.parse()?)
            }
        }
    };
}
