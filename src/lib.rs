use std::path::Path;
fn get_xattr(path: &Path, key: &str) -> Option<String> {
    xattr::get(path, key)
        .ok()
        .flatten()
        .and_then(|v| String::from_utf8(v).ok())
}

/// Parse Comma-delimited string into a Vec<&str>
fn csv(input: &str) -> Vec<&str> {
    input.split(",").collect()
}

pub struct Attributes {
    /// Comment extended attribute for the file.
    ///
    // user.xdg.comment
    pub comment: Option<String>,

    /// The program that created the file.
    // user.xdg.creator
    pub creator: Option<String>,

    /// The language the file is written in.
    // user.xdg.language
    pub language: Option<String>,

    /// Tags for the file.
    ///
    /// Not actually a part of the XDG spec, but is a de-facto standard
    /// in various popular file managers.
    ///
    /// The tags are stored as a comma-delimited string called `user.xdg.tags`.
    pub tags: Option<Vec<String>>,

    // origin attributes
    // todo: Probably make this a new struct?
    /// The URL the file was downloaded from.
    // user.xdg.origin.url
    pub origin_url: Option<String>,

    /// Referrer URL
    // user.xdg.referrer.url
    pub origin_referrer: Option<String>,

    /// Email subject line.
    // user.xdg.origin.email.subject
    pub origin_email_subject: Option<String>,

    /// Email sender.
    // user.xdg.origin.email.from
    pub origin_email_from: Option<String>,

    /// Email message ID
    // user.xdg.origin.email.message-id
    pub origin_email_message_id: Option<String>,
}

impl Attributes {
    pub fn read_file(path: &Path) -> Result<Self, std::io::Error> {
        let comment = get_xattr(path, "user.xdg.comment");
        let creator = get_xattr(path, "user.xdg.creator");
        let language = get_xattr(path, "user.xdg.language");
        let tags = get_xattr(path, "user.xdg.tags");
        let origin_url = get_xattr(path, "user.xdg.origin.url");
        let origin_referrer = get_xattr(path, "user.xdg.referrer.url");
        let origin_email_subject = get_xattr(path, "user.xdg.origin.email.subject");
        let origin_email_from = get_xattr(path, "user.xdg.origin.email.from");
        let origin_email_message_id = get_xattr(path, "user.xdg.origin.email.message-id");

        Ok(Self {
            comment,
            creator,
            language,
            tags: tags.map(|v| csv(&v).iter().map(|s| s.to_string()).collect()),
            origin_url,
            origin_referrer,
            origin_email_subject,
            origin_email_from,
            origin_email_message_id,
        })
    }

    pub fn write_file(&self, path: &Path) -> Result<(), std::io::Error> {
        if let Some(ref comment) = self.comment {
            xattr::set(path, "user.xdg.comment", comment.as_bytes())?;
        }
        if let Some(ref creator) = self.creator {
            xattr::set(path, "user.xdg.creator", creator.as_bytes())?;
        }
        if let Some(ref language) = self.language {
            xattr::set(path, "user.xdg.language", language.as_bytes())?;
        }
        if let Some(ref tags) = self.tags {
            let tags_str = tags.join(",");
            xattr::set(path, "user.xdg.tags", tags_str.as_bytes())?;
        }
        if let Some(ref origin_url) = self.origin_url {
            xattr::set(path, "user.xdg.origin.url", origin_url.as_bytes())?;
        }
        if let Some(ref origin_referrer) = self.origin_referrer {
            xattr::set(path, "user.xdg.referrer.url", origin_referrer.as_bytes())?;
        }
        if let Some(ref origin_email_subject) = self.origin_email_subject {
            xattr::set(path, "user.xdg.origin.email.subject", origin_email_subject.as_bytes())?;
        }
        if let Some(ref origin_email_from) = self.origin_email_from {
            xattr::set(path, "user.xdg.origin.email.from", origin_email_from.as_bytes())?;
        }
        if let Some(ref origin_email_message_id) = self.origin_email_message_id {
            xattr::set(path, "user.xdg.origin.email.message-id", origin_email_message_id.as_bytes())?;
        }
        Ok(())
    }
}
