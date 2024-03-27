/// Trait for database entities.
pub(crate) trait Entity {
    fn table_name() -> &'static str;
}

/// Quickly implement `Entity` trait on a struct.
#[allow(unused_macros)]
macro_rules! impl_entity {
    ($sn:ident, $tn:literal) => {
        impl Entity for $sn {
            fn table_name() -> &'static str {
                $tn
            }
        }
    };
}

// define your entities here
// NOTE: helpful derive macros:
// Debug, Clone, sqlx::FromRow, serde::{Serialize, Deserialize}, poem_openapi::Object]

// use impl_entity! to quickly implement Entity trait for a struct
