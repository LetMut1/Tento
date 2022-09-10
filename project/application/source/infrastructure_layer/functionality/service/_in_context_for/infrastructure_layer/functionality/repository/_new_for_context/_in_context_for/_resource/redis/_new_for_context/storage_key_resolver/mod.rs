pub struct StorageKeyResolver;

impl StorageKeyResolver {                                           // TODO УДалить или закомментировать при переходе на Постгер
    const PREFIX_1: &'static str = "1:";
    const PREFIX_2: &'static str = "2:";
    const PREFIX_3: &'static str = "3:";
    const PREFIX_4: &'static str = "4:";
    const PREFIX_5: &'static str = "5:";
    const PREFIX_6: &'static str = "6:";

    pub fn get_1<'a>(
        application_user_email: &'a str
    ) -> String {
        return Self::PREFIX_1.to_string() + application_user_email;
    }

    pub fn get_2<'a>(
        application_user_id: i64,
        application_user_log_in_token_device_id: &'a str
    ) -> String {
        return Self::PREFIX_2.to_string() + application_user_id.to_string().as_str()  + ":" + application_user_log_in_token_device_id;
    }

    pub fn get_3(
        application_user_id: i64,
    ) -> String {
        return Self::PREFIX_3.to_string() + application_user_id.to_string().as_str();
    }

    pub fn get_4<'a>(
        application_user_access_token_id: &'a str
    ) -> String {
        return Self::PREFIX_4.to_string() + application_user_access_token_id;
    }

    pub fn get_5<'a>(
        application_user_id: i64,
        application_user_log_in_token_device_id: &'a str,
    ) -> String {
        return Self::PREFIX_5.to_string() + application_user_id.to_string().as_str() + ":" + application_user_log_in_token_device_id;
    }

    pub fn get_6(
        application_user_id: i64
    ) -> String {
        return Self::PREFIX_6.to_string() + application_user_id.to_string().as_str();
    }
}