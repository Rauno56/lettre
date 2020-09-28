use std::fs;

use lettre::{
    transport::smtp::authentication::Credentials,
    transport::smtp::client::{Certificate, Tls, TlsParameters},
    Message, SmtpTransport, Transport,
};

fn main() {
    tracing_subscriber::fmt::init();

    let email = Message::builder()
        .from("NoBody <nobody@domain.tld>".parse().unwrap())
        .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
        .to("Hei <hei@domain.tld>".parse().unwrap())
        .subject("Happy new year")
        .body("Be happy!")
        .unwrap();

    let pem_cert = fs::read("certificate.pem").unwrap();
    let cert = Certificate::from_pem(&pem_cert).unwrap();
    let mut tls = TlsParameters::builder("smtp.server.com".into());
    tls.add_root_certificate(cert);
    let tls = tls.build().unwrap();

    let creds = Credentials::new("smtp_username".to_string(), "smtp_password".to_string());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::builder_dangerous("smtp.server.com")
        .port(465)
        .tls(Tls::Wrapper(tls))
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}