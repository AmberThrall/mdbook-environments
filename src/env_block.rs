use super::EnvInfo;
use mdbook::errors::Result as MdbookResult;

pub struct EnvBlock {
    pub info: EnvInfo,
    pub body: String, 
}

impl EnvBlock {
    pub fn parse(info_string: &str, content: &str) -> MdbookResult<Self> {
        let body = extract_block_body(content);

        let info = EnvInfo::parse(info_string)?;
        Ok(Self {
            info,
            body: String::from(body),
        })
    }
}

fn extract_block_body(content: &str) -> &str {
    const PRE_END: char = '\n';
    const POST: &str = "```";

    let start_index = content.find(PRE_END)
        .map(|index| index + 1)
        .unwrap_or_default();
    let end_index = content.len() - POST.len();

    let content = &content[start_index..end_index];
    content.trim()
}   