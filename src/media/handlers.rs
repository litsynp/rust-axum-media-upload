use axum::body::Bytes;
use axum::extract::Multipart;
use std::fmt::{Display, Formatter};

// @formatter:off
#[utoipa::path(
    post,
    operation_id = "upload_media",
    path = "/media",
    tag = "media",
    request_body(content = UploadMediaRequest, content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "Media uploaded"),
    ),
)] // @formatter:on
pub async fn upload_media(mut multipart: Multipart) {
    while let Some(file) = multipart.next_field().await.unwrap() {
        let file = File {
            name: file.file_name().unwrap().to_string(),
            data: file.bytes().await.unwrap(),
        };

        tracing::info!("{}", file);
    }
}

struct File {
    name: String,
    data: Bytes,
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "file: {}, size: {} bytes ({}MB)",
            self.name,
            self.data.len(),
            self.data.len() as f64 / 1024.0 / 1024.0
        )
    }
}
