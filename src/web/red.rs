#[get("/red")]
pub fn red() -> &'static str {  // <- request handler
    "red front page"
}
