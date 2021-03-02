pub struct StandartResponseBodyWrapper;

impl StandartResponseBodyWrapper {
    // pub fn create_for_success(&'static str) ->  // TODO 
    pub fn create_for_fail(code: &'static str) -> String {
        return "{\"success\":false, \"code\":".to_string() + code + "\"}"
    }
}