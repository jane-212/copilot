mod jina;
mod mail;
mod openai;

use std::env;

use anyhow::Result;
use jina::Jina;
use mail::Mailer;
use openai::Openai;
use tera::Tera;

macro_rules! templates {
    ($($name:expr),*) => {
        [
            $(
                ($name, include_str!(concat!("../../templates/", $name))),
            )*
        ]
    };
}

pub struct Helper {
    pub mailer: Mailer,
    pub openai: Openai,
    pub deep_seek: Openai,
    pub tera: Tera,
    pub jina: Jina,
}

impl Helper {
    pub fn new() -> Result<Self> {
        let mailer = Self::new_mailer()?;
        let openai = Self::new_openai()?;
        let deep_seek = Self::new_deep_seek()?;
        let tera = Self::new_tera()?;
        let jina = Jina::new();
        let helper = Self {
            mailer,
            openai,
            deep_seek,
            tera,
            jina,
        };

        Ok(helper)
    }

    fn new_tera() -> Result<Tera> {
        let templates = templates!("base.html", "error.html", "normal.html");
        let mut tera = Tera::default();
        for (name, content) in templates {
            log::info!("添加模版: {name}");
            tera.add_raw_template(name, content)?;
        }

        Ok(tera)
    }

    fn new_deep_seek() -> Result<Openai> {
        let deep_seek_key = env::var("DEEP_SEEK_KEY")?;
        log::info!("找到了deep seek key");
        let deep_seek = Openai::new("https://api.deepseek.com", "deepseek-chat", deep_seek_key);

        Ok(deep_seek)
    }

    fn new_openai() -> Result<Openai> {
        let openai_key = env::var("OPENAI_KEY")?;
        log::info!("找到了openai key");
        let openai = Openai::new(
            "https://models.inference.ai.azure.com",
            "gpt-4o",
            openai_key,
        );

        Ok(openai)
    }

    fn new_mailer() -> Result<Mailer> {
        let to = env::var("MAIL_TO")?;
        log::info!("找到了mail to: {}", to);
        let username = env::var("MAIL_USERNAME")?;
        log::info!("找到了mail username: {}", username);
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
