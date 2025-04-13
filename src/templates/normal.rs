use askama::Template;
use bon::bon;

#[derive(Template)]
#[template(path = "normal.html")]
pub struct Normal {
    star: u32,
    download: i64,
}

#[bon]
impl Normal {
    #[builder]
    pub fn new(star: u32, download: i64) -> Self {
        Self { star, download }
    }
}
