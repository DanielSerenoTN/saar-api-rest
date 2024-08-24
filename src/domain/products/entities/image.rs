#[derive(Serialize, Deserialize)]
struct Image {
    id: String,
    url: String,
    description: String,
    default: bool,
}
