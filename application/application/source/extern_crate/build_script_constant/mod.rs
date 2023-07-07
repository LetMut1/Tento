pub mod environment_configuration {
    pub const ENVIRONMENT_CONFIGURATION_CONSTANT_NAME: &'static str = "ENVIRONMENT_CONFIGURATION";

    #[macro_export]
    macro_rules! ENVIRONMENT_CONFIGURATION_CONSTANT_MODULE_NAME {
        () => {
            "environment_configuration_constant"
        };
    }
}
