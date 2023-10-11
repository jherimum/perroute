#[macro_export]
macro_rules! into_event {
    ($command: ty) => {
        impl perroute_messaging::events::IntoEvent for $command {
            fn into_event(&self) -> Option<perroute_messaging::events::Event> {
                None
            }
        }
    };
    ($command: ty, $event_type: expr, $id: expr) => {
        impl perroute_messaging::events::IntoEvent for $command {
            fn into_event(&self) -> Option<perroute_messaging::events::Event> {
                Some(perroute_messaging::events::Event::new(
                    $id(self),
                    $event_type,
                ))
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
        impl $crate::query_bus::queries::Query for $cmd {
            fn ty(&self) -> QueryType {
                $ty
            }
        }
    };
}
