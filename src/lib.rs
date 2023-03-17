#[macro_use]
extern crate serde;
extern crate tinytemplate;

mod env_info;
mod env_block;
mod env;

pub use env_info::*;
pub use env_block::*;
pub use env::*;

use mdbook::{
    book::{Book, BookItem},
    errors::Result as MdbookResult,
    preprocess::{Preprocessor, PreprocessorContext},
};
use pulldown_cmark::{CodeBlockKind::*, Event, Options, Parser, Tag, HeadingLevel};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Counter {
    pub chapter: Option<usize>,
    pub section: Option<usize>,
    pub env: usize,
}

impl Counter {
    pub fn next_chapter(&mut self) {
        self.chapter = match self.chapter {
            Some(c) => Some(c + 1),
            None => Some(1),
        };
        self.section = match self.section {
            Some(_) => Some(1),
            None => None,
        };
        self.env = 1;
    }

    pub fn next_section(&mut self) {
        self.section = match self.section {
            Some(c) => Some(c + 1),
            None => Some(1),
        };
        self.env = 1;
    }

    pub fn next_env(&mut self) {
        self.env += 1;
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self {
            chapter: None,
            section: None,
            env: 1,
        }
    }
}

impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(chapter) = self.chapter {
            write!(f, "{}.", chapter)?;
        }
        if let Some(section) = self.section {
            write!(f, "{}.", section)?;
        }
        write!(f, "{}", self.env)
    }
}

pub struct EnvPreprocessor<'a> {
    pub environments: Environments<'a>,
}

impl<'a> Default for EnvPreprocessor<'a> {
    fn default() -> Self {
        Self {
            environments: Environments::default(),
        }
    }
}

impl<'a> Preprocessor for EnvPreprocessor<'a> {
    fn name(&self) -> &str {
        "env-preprocessor"
    }
    
    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> MdbookResult<Book> {
        let mut res = None;
        book.for_each_mut(|item: &mut BookItem| {
            if let Some(Err(_)) = res {
                return;
            }

            if let BookItem::Chapter(ref mut chapter) = *item {
                res = Some(self.preprocess(&chapter.content).map(|md| {
                    chapter.content = md;
                }));
            }
        });

        res.unwrap_or(Ok(())).map(|_| book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }   
}

impl<'a> EnvPreprocessor<'a> {
    fn preprocess(&self, content: &str) -> MdbookResult<String> {
        let mut opts = Options::empty();
        opts.insert(Options::ENABLE_TABLES);
        opts.insert(Options::ENABLE_FOOTNOTES);
        opts.insert(Options::ENABLE_STRIKETHROUGH);
        opts.insert(Options::ENABLE_TASKLISTS);

        let mut env_blocks = vec![];
        let mut counters: HashMap<String, Counter> = self.environments.get_counters();

        let events = Parser::new_ext(content, opts);
        for (e, span) in events.into_offset_iter() {
            match e.clone() {
                Event::Start(Tag::Heading(lvl, _, _)) => {
                    for (_, counter) in counters.iter_mut() {
                        match lvl {
                            HeadingLevel::H1 => counter.next_chapter(),
                            HeadingLevel::H2 => counter.next_section(),
                            _ => {},
                        }
                    }
                },
                Event::Start(Tag::CodeBlock(Fenced(info_string))) => {
                    let span_content = &content[span.start..span.end];
                    let env_block = match EnvBlock::parse(info_string.as_ref(), span_content) {
                        Ok(block) => block,
                        Err(_) => continue,
                    };
    
                    let mut counter = Counter::default();
                    if let Some(env) = self.environments.get(&env_block.info.name) {
                        if let Some(ref counter_id) = env.counter_id {
                            match counters.get_mut(counter_id) {
                                Some(v) => {
                                    counter = *v;
                                    v.next_env();
                                }
                                None => {},
                            }
                        }
                    }
    
                    let rendered = match self.environments.render(&env_block, &counter) {
                        Ok(content) => content,
                        Err(e) => format!("{}", e),
                    };
    
                    env_blocks.push((span, rendered));
                },
                _ => {},
            }
        }

        let mut content = content.to_string();
        for (span, block) in env_blocks.iter().rev() {
            let pre_content = &content[..span.start];
            let post_content = &content[span.end..];
            content = format!("{}\n{}{}", pre_content, block, post_content);
        }
    
        Ok(content)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn prep(content: &str) -> String {
        let mut preprocessor = EnvPreprocessor::default();
        preprocessor.environments.register_builtin(BuiltinEnvironments::Proof);

        preprocessor.preprocess(content).unwrap()
    }

    #[test]
    fn theorem() {
        let content = r#"
```theorem "Theorem Name"
body
```
"#;

        let expected = r#"
???
"#;
        assert_eq!(expected, prep(content));
    }

    #[test]
    fn proof() {
        let content = r#"
```proof
The result is trivial.
```
"#;

        let expected = r#"
???
"#;
        assert_eq!(expected, prep(content));
    }
}