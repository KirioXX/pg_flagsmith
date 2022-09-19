use pgx::*;
use flagsmith::{Flagsmith, FlagsmithOptions};

pg_module_magic!();

#[pg_extern]
fn is_feature_enabled(flagsmith_key: &str, flag_name: &str) -> bool {
    let options = FlagsmithOptions {..Default::default()};
    let flagsmith = Flagsmith::new(
        flagsmith_key.to_string(),
        options,
    );
    let flags = flagsmith.get_environment_flags().unwrap();
    flags.is_feature_enabled(flag_name).unwrap()
}

#[pg_extern]
fn is_feature_enabled_for_identity(flagsmith_key: &str, identifier: &str, flag_name: &str) -> bool {
    let options = FlagsmithOptions {..Default::default()};
    let flagsmith = Flagsmith::new(
        flagsmith_key.to_string(),
        options,
    );
    let identity_flags = flagsmith.get_identity_flags(identifier, None).unwrap();
    identity_flags.is_feature_enabled(flag_name).unwrap()
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_is_feature_enabled() {
        assert_eq!(true,
                   crate::is_feature_enabled("test-key", "test-flag"));
    }

    #[pg_test]
    fn test_is_feature_enabled_for_identity() {
        assert_eq!(true,
                   crate::is_feature_enabled_for_identity("test-key", "bob@example.com", "test-flag"));
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
