mod logger;
mod mail;
mod openai;

use std::env;

use anyhow::Result;
use logger::Logger;
use mail::Mailer;
use openai::Openai;
use tera::Tera;
use tokio::sync::Mutex;

pub struct Helper {
    pub mailer: Mailer,
    pub openai: Openai,
    pub deep_seek: Openai,
    pub logger: Mutex<Logger>,
    pub tera: Tera,
}

impl Helper {
    pub fn new() -> Result<Self> {
        let mailer = Self::new_mailer()?;
        let openai = Self::new_openai()?;
        let deep_seek = Self::new_deep_seek()?;
        let logger = Logger::new();
        let tera = Self::new_tera()?;
        let helper = Self {
            mailer,
            openai,
            deep_seek,
            logger: Mutex::new(logger),
            tera,
        };

        Ok(helper)
    }

    fn new_tera() -> Result<Tera> {
        let mut tera = Tera::default();
        tera.add_raw_template("base.html", include_str!("../../templates/base.html"))?;
        tera.add_raw_template("log.html", include_str!("../../templates/log.html"))?;

        Ok(tera)
    }

    fn new_deep_seek() -> Result<Openai> {
        let deep_seek_key = env::var("DEEP_SEEK_KEY")?;
        let deep_seek = Openai::new("https://api.deepseek.com", "deepseek-chat", deep_seek_key)?;

        Ok(deep_seek)
    }

    fn new_openai() -> Result<Openai> {
        let openai_key = env::var("OPENAI_KEY")?;
        let openai = Openai::new(
            "https://models.inference.ai.azure.com",
            "gpt-4o",
            openai_key,
        )?;

        Ok(openai)
    }

    fn new_mailer() -> Result<Mailer> {
        let to = env::var("MAIL_TO")?;
        let username = env::var("MAIL_USERNAME")?;
        let password = env::var("MAIL_PASSWORD")?;
        let mailer = Mailer::new(
            format!("Bot <{username}>"),
            to,
            "smtp.163.com",
            username,
            password,
        )?;

        Ok(mailer)
    }
}
