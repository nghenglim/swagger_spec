use schema::Schema;
#[derive(Clone, Debug)]
pub struct RequestBody {
    description: String,
    content: RequestBodyContent,
}
#[derive(Clone, Debug)]
pub struct RequestBodyContent {
    application_json: Option<RequestBodyContentMediaTypeObject>,
}
#[derive(Clone, Debug)]
pub struct RequestBodyContentMediaTypeObject {
    schema: Option<Schema>,
}