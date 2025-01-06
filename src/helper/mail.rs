use anyhow::Result;
use lettre::{
    message::{header::ContentType, IntoBody, Mailbox},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

pub struct Mailer {
    client: AsyncSmtpTransport<Tokio1Executor>,
    from: Mailbox,
    to: Mailbox,
}

impl Mailer {
    pub fn new(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        smtp_host: impl AsRef<str>,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<Self> {
        let creds = Credentials::new(username.into(), password.into());
        let client = AsyncSmtpTransport::<Tokio1Executor>::relay(smtp_host.as_ref())?
            .credentials(creds)
            .build();
        let from = from.as_ref().parse()?;
        let to = to.as_ref().parse()?;
        let mailer = Self { client, from, to };

        Ok(mailer)
    }

    pub async fn send(&self, subject: impl Into<String>, body: impl IntoBody) -> Result<()> {
        let email = Message::builder()
            .from(self.from.clone())
            .to(self.to.clone())
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(body)?;
        self.client.send(email).await?;

        Ok(())
    }
}
