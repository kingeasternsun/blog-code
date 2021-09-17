use pgx::*;
use serde::{Deserialize, Serialize};

pg_module_magic!();

#[pg_extern]
fn hello_my_schema() -> &'static str {
    "Hello, my_schema"
}


#[derive(PostgresType, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct SomeStruct {}

#[pg_extern]
#[search_path(@extschema@)]
fn return_vec_of_customtype() -> Vec<SomeStruct> {
    vec![SomeStruct {}]
}

#[derive(PostgresType, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct SomeType {a:i32}

#[pg_extern]
#[search_path(schema_a, schema_b, public)]
fn return_vec_of_customtp() -> Vec<SomeType> {
    vec![SomeType {a:3}]
}



#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_hello_my_schema() {
        assert_eq!("Hello, my_schema", crate::hello_my_schema());
    }

}



#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
