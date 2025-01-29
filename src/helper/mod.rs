mod jina;
mod mail;
mod openai;

use std::env;

use anyhow::Result;
use jina::Jina;
use mail::Mailer;
use openai::Openai;

pub struct Helper {
    pub mailer: Mailer,
    pub openai: Openai,
    pub deep_seek: Openai,
    pub jina: Jina,
}

impl Helper {
    pub fn new() -> Result<Self> {
        let mailer = Self::new_mailer()?;
        let openai = Self::new_openai()?;
        let deep_seek = Self::new_deep_seek()?;
        let jina = Jina::new();
        let helper = Self {
            mailer,
            openai,
            deep_seek,
            jina,
        };

        Ok(helper)
    }

    fn new_deep_seek() -> Result<Openai> {
        let deep_seek_key = env::var("DEEP_SEEK_KEY")?;
        log::info!("找到了deep seek key");
        let deep_seek = Openai::builder()
            .base("https://api.deepseek.com")
            .model("deepseek-chat")
            .key(deep_seek_key)
            .build();

        Ok(deep_seek)
    }

    fn new_openai() -> Result<Openai> {
        let openai_key = env::var("OPENAI_KEY")?;
        log::info!("找到了openai key");
        let openai = Openai::builder()
            .base("https://models.inference.ai.azure.com")
            .model("gpt-4o")
            .key(openai_key)
            .build();

        Ok(openai)
    }

    fn new_mailer() -> Result<Mailer> {
        let to = env::var("MAIL_TO")?;
        log::info!("找到了mail to: {}", to);
        let username = env::var("MAIL_USERNAME")?;
        log::info!("找到了mail username: {}", username);
        let password = env::var("MAIL_PASSWORD")?;
        let mailer = Mailer::builder()
            .from(format!("Bot <{username}>"))
            .to(to)
            .server("smtp.163.com")
            .username(username)
            .password(password)
            .build()?;

        Ok(mailer)
    }
}
