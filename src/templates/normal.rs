use askama::Template;
use bon::bon;

#[derive(Template)]
#[template(path = "normal.html")]
pub struct Normal<'a> {
    star: u32,
    download: i64,
    challenge: &'a str,
}

#[bon]
impl<'a> Normal<'a> {
    #[builder]
    pub fn new(star: u32, download: i64, challenge: &'a str) -> Self {
        Self {
            star,
            download,
            challenge,
        }
    }
}
