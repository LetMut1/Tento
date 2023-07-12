pub mod environment_configuration {
    pub const ENVIRONMENT_CONFIGURATION_CONSTANT_NAME: &'static str = "ENVIRONMENT_CONFIGURATION";

    #[macro_export]
    macro_rules! environment_configuration_constant_file_name {
        () => {
            "environment_configuration_constant.rs"
        };
    }

    #[macro_export]
    macro_rules! build_environment_configuration {
        () => {
            include!(
                concat!(
                    env!("OUT_DIR"),
                    concat!(
                        "/",
                        environment_configuration_constant_file_name!()
                    )
                )
            );
        };
    }
}