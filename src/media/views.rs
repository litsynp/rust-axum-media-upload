use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct UploadMediaRequest {
    #[schema(format = Binary)]
    #[allow(dead_code)]
    file: String,
}
