use askama::Template;
use bon::bon;

#[derive(Template)]
#[template(path = "normal.html")]
pub struct Normal<'a> {
    zh: &'a str,
    en: &'a str,
    news: &'a [New],
}

pub struct New {
    index: usize,
    title: String,
}

#[bon]
impl New {
    #[builder]
    pub fn new(index: usize, title: String) -> New {
        New { index, title }
    }
}

#[bon]
impl<'a> Normal<'a> {
    #[builder]
    pub fn new(zh: &'a str, en: &'a str, news: &'a [New]) -> Normal<'a> {
        Normal { zh, en, news }
    }
}
