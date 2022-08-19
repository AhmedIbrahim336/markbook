use std::borrow::Cow;

use regex::{Captures, Match, Regex};

#[derive(Debug)]
pub enum BlockToken {
    H1 {
        text: String,
        inline_tokens: Vec<InlineToken>,
    },
    H2 {
        text: String,
        inline_tokens: Vec<InlineToken>,
    },
    H3 {
        text: String,
        inline_tokens: Vec<InlineToken>,
    },
    H4 {
        text: String,
        inline_tokens: Vec<InlineToken>,
    },
    H5 {
        text: String,
        inline_tokens: Vec<InlineToken>,
    },
    H6 {
        text: String,
        inline_tokens: Vec<InlineToken>,
    },
    P {
        text: String,
        inline_tokens: Vec<InlineToken>,
    },
    Bold,
    Italic,
    Anchor,
    Img,
}

#[derive(Debug)]
pub enum InlineToken {
    Link {
        href: String,
        text: String,
        raw: String,
    }, // reg: \[(?P<text>[^\]]+)\]\((?P<href>[^\]]+)\)
    Bold {
        value: String,
        raw: String,
    },
    Code {
        value: String,
        raw: String,
    },
    Italic {
        value: String,
        raw: String,
    },
    Image {
        alt: String,
        src: String,
        raw: String,
    }
}

impl InlineToken {
    fn extract(text: &mut String) -> Vec<InlineToken> {
        // Match any one of these
        let re_set = [
            r"\[(?P<link_text>[^\]]+)\]\((?P<href>[^\]]+)\)", // Link
            r"\*\*(?P<bold>[^\*]+)\*\*",                      // Bold text
            r"_(?P<italic>[^_]+)_",                           // Italic text
            r"`(?P<code>[^`]+)`",                             // Inline code
            r"!\[(?P<alt>[^\]]+)\]\((?P<src>[^\]]+)\)" // Image
        ];

        let re = Regex::new(&re_set.join("|")).unwrap();

        let tokens = re
            .captures_iter(text)
            .enumerate()
            .map(|(idx, caps)| {
                let raw = caps[0].to_string();

                let href = InlineToken::get_name(&caps, "href");
                let link_text = InlineToken::get_name(&caps, "link_text");
                let bold = InlineToken::get_name(&caps, "bold");
                let italic = InlineToken::get_name(&caps, "italic");
                let code = InlineToken::get_name(&caps, "code");
                let img_src = InlineToken::get_name(&caps, "src");
                let img_alt = InlineToken::get_name(&caps, "alt");

                if href.is_some() && link_text.is_some() {
                    InlineToken::Link {
                        href: href.unwrap(),
                        text: link_text.unwrap(),
                        raw,
                    }
                } else if img_src.is_some() && img_alt.is_some() {
                    InlineToken::Image {
                        src: img_src.unwrap(),
                        alt: img_alt.unwrap(),
                        raw
                    }
                } else if bold.is_some() {
                    InlineToken::Bold {
                        value: bold.unwrap(),
                        raw,
                    }
                } else if italic.is_some() {
                    InlineToken::Italic {
                        value: italic.unwrap(),
                        raw,
                    }
                } else if code.is_some() {
                    InlineToken::Code {
                        value: code.unwrap(),
                        raw,
                    }
                } else {
                    // Should never happen
                    // Regex should never match other names
                    panic!("Regex maching unsupported type")
                }
            })
            .collect();

        tokens
    }


    fn get_name(caps: &Captures, name: &str) -> Option<String> {
        if caps.name(name).is_some() {
            return Some(caps[name].to_string());
        }

        None
    }

    fn get_raw(&self) -> &String {
        match &self {
            InlineToken::Link { raw, .. } => raw,
            InlineToken::Bold {  raw, .. } => raw,
            InlineToken::Code { raw, .. } => raw,
            InlineToken::Italic { raw, .. } => raw,
            InlineToken::Image { raw, .. } => raw
        }
    }

    fn mask_tokens(mut text: String, tokens: &Vec<InlineToken>) -> String {
        tokens.iter().enumerate().for_each(|(idx, token)| {
            text = text.replace(token.get_raw(), &format!("<${}>", idx + 1))
        });

        text
    }
}

#[derive(Debug)]
pub enum HeadingType {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl HeadingType {
    pub fn new(line: &str) -> Option<HeadingType> {
        let re = Regex::new(r"^(?P<type>#{1,})").unwrap();

        if let Some(caps) = re.captures(line) {
            let number_signs = &caps["type"];

            let h_type = match number_signs.len() {
                1 => HeadingType::H1,
                2 => HeadingType::H2,
                3 => HeadingType::H3,
                4 => HeadingType::H4,
                5 => HeadingType::H5,
                _ => HeadingType::H6,
            };

            return Some(h_type);
        }
        None
    }
}

#[derive(Debug)]
pub struct Heading {
    h_type: HeadingType,
    text: String,
    inline_tokens: Vec<InlineToken>,
}

impl Heading {
    pub fn new(line: &str) -> Option<Heading> {
        let line = line.trim();
        let h_type = match HeadingType::new(line) {
            Some(h) => h,
            None => return None,
        };

        // Extract text
        let re = Regex::new(r"#{1,6}\s+(?P<text>.+)").unwrap();

        if let Some(caps) = re.captures(line) {
            let mut text = (&caps["text"]).to_string();
            let inline_tokens = InlineToken::extract(&mut text);

            return Some(Heading {
                h_type,
                text: InlineToken::mask_tokens(text, &inline_tokens),
                inline_tokens,
            });
        }

        None
    }
}


#[derive(Debug)]
pub struct Paragraph {
    text: String,
    inline_tokens: Vec<InlineToken>
}

impl Paragraph {
    pub fn new(line: &str) -> Option<Self> {
        let mut text = line.trim().to_string();
        if text.len() == 0 {
            return None
        }

        let inline_tokens = InlineToken::extract(&mut text);
        let text = InlineToken::mask_tokens(text, &inline_tokens);

        Some(Paragraph { text, inline_tokens })
    }
}