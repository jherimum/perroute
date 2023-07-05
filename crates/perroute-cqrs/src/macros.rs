#[macro_export]
macro_rules! command {
    ($name: tt, $cmd_type: expr, $($fname: ident : $ftype: ty),*) => {
        #[derive(Debug, serde::Serialize, Clone, PartialEq, Eq, derive_builder::Builder, derive_getters::Getters)]
        pub struct $name {
            $($fname : $ftype),*
        }
        $crate::impl_command!($name, $cmd_type);
    };

}

#[macro_export]
macro_rules! impl_command {
    ($cmd: ty, $ty: expr) => {
        impl Command for $cmd {
            fn ty(&self) -> CommandType {
                $ty
            }
        }
    };
}

#[macro_export]
macro_rules! query {
    ($name: tt, $query_type: expr, $($fname: ident : $ftype: ty),*) => {
        #[derive(Debug, serde::Serialize, Clone, PartialEq, Eq, derive_builder::Builder, derive_getters::Getters)]
        pub struct $name {
            $($fname : $ftype),*
        }
        $crate::impl_query!($name, $query_type);
    };
    ($name: tt, $query_type: expr) => {
        #[derive(Debug, serde::Serialize, Clone, PartialEq, Eq, derive_builder::Builder, derive_getters::Getters)]
        pub struct $name {

        }
        $crate::impl_query!($name, $query_type);
    };

}

#[macro_export]
macro_rules! impl_query {
    ($cmd: ty, $ty: expr) => {
        impl Query for $cmd {
            fn ty(&self) -> QueryType {
                $ty
            }
        }
    };
}
