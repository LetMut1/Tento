use {
    super::Formatter,
    crate::infrastructure_layer::data::aggregate_error::InvalidArgument,
    std::marker::PhantomData,
};
impl Formatter<PhantomData<InvalidArgument>> {
    pub fn format() -> String {
        return "Invalid argument".to_string();
    }
}
