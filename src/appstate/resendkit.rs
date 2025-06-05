use resend_rs::{types::CreateEmailBaseOptions, Resend};

#[derive(Clone)]
pub struct ResendKit {
    resend: Resend,
    from: String,
}

#[derive(Debug)]
pub struct ScheduledLetter {
    pub letter_id: String,
    pub email_id: String,
}

impl ResendKit {
    pub fn initialize() -> Self {
        let resend_api_key = std::env::var("RESEND_API_KEY").expect("No Resend API key found");
        
        Self {
            resend: Resend::new(&resend_api_key),
            from: "noreply <goshen@docklabs.xyz>".to_string()
        }
    }

    pub async fn send_verification_email(&self, email: &str, otp: String) -> Result<(), resend_rs::Error> {
        let to = [email];
        let subject = "Verication Code";

        let body = format!("
            <p>Hello there!</p>
            <p>Your OTP is : {otp}</p>
        ");
        let mail = CreateEmailBaseOptions::new(self.from.as_str(), to, subject)
            .with_html(&body);

        self.resend.emails
            .send(mail)
            .await
            .map(|_| ())
    }

    pub async fn schedule_email(&self, letter: crate::appstate::Letter) -> Result<ScheduledLetter,resend_rs::Error> {
        let to = [letter.writer_email];
        let subject = "Letter from the past you";
        let from = format!("{} <goshen@docklabs.xyz>", letter.writer_name);

        let due_date = letter.due_date
            .try_to_rfc3339_string()
            .inspect_err(|e| eprintln!("Failed to format due_date for letter {}: {}", &letter.id, e))
            .map_err(|_| resend_rs::Error::Parse("Error parsing the datetime".to_string()))?;        

        let mail = CreateEmailBaseOptions::new(from, to, subject)
            .with_text(&letter.message)
            .with_scheduled_at(&due_date);

        self.resend.emails
            .send(mail)
            .await
            .inspect_err(|e| eprintln!("Error scheduling email for letter {}: {}", &letter.id, e))
            .map(|res| {
                ScheduledLetter {
                    letter_id: letter.id,
                    email_id: res.id.to_string()
                }
            })
    }
}