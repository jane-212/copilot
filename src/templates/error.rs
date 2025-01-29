use askama::Template;
use bon::bon;

#[derive(Template)]
#[template(path = "error.html")]
pub struct Error<'a> {
    error_lines: Vec<&'a str>,
}

#[bon]
impl<'a> Error<'a> {
    #[builder]
    pub fn new(error_lines: Vec<&'a str>) -> Error<'a> {
        Error { error_lines }
    }
}
