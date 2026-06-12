use std::sync::LazyLock;

use regex::Regex;
use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub struct Email(String);

impl Email {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Email {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let v = String::deserialize(deserializer)?;
        if validate_email(&v) {
            Ok(Self(v))
        } else {
            Err(serde::de::Error::custom("Invalid email"))
        }
    }
}

static EMAIL_USER_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+\z").unwrap());
static EMAIL_DOMAIN_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
    ).unwrap()
});
// literal form, ipv4 or ipv6 address (SMTP 4.1.3)
// static EMAIL_LITERAL_RE: LazyLock<Regex> =
//     LazyLock::new(|| Regex::new(r"\[([a-fA-F0-9:\.]+)\]\z").unwrap());

#[must_use]
fn validate_domain_part(domain_part: &str) -> bool {
    if EMAIL_DOMAIN_RE.is_match(domain_part) {
        return true;
    }

    // maybe we have an ip as a domain?
    // match EMAIL_LITERAL_RE.captures(domain_part) {
    //     Some(caps) => match caps.get(1) {
    //         Some(c) => c.as_str().validate_ip(),
    //         None => false,
    //     },
    //     None => false,
    // }
    false
}

fn validate_email(val: &str) -> bool {
    if val.is_empty() || !val.contains('@') {
        return false;
    }

    let parts: Vec<&str> = val.rsplitn(2, '@').collect();
    let user_part = parts[1];
    let domain_part = parts[0];

    // validate the length of each part of the email, BEFORE doing the regex
    // according to RFC5321 the max length of the local part is 64 characters
    // and the max length of the domain part is 255 characters
    // https://datatracker.ietf.org/doc/html/rfc5321#section-4.5.3.1.1
    if user_part.chars().count() > 64 || domain_part.chars().count() > 255 {
        return false;
    }

    if !EMAIL_USER_RE.is_match(user_part) {
        return false;
    }

    if !validate_domain_part(domain_part) {
        // Still the possibility of an [IDN](https://en.wikipedia.org/wiki/Internationalized_domain_name)
        // return match domain_to_ascii(domain_part) {
        //     Ok(d) => validate_domain_part(&d),
        //     Err(_) => false,
        // };
        return false;
    }

    true
}
