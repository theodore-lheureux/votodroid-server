use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

impl FieldError {
    pub fn new(field: String, message: String) -> Self {
        Self { field, message }
    }
}
