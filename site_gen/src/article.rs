use std::{fs::File, io::Read, str::FromStr};

use crate::{layout::main_layout, markdown::markdown_to_html};
use chrono::{DateTime, Datelike, TimeZone, Utc};
use html_node::{pretty::Pretty, text, typed::elements::*, unsafe_text};
use pulldown_cmark::{Event, Tag, TagEnd};
use serde::{Deserialize, Serialize};

pub fn get_article_page(article_metadata: &ArticleMetadata<Utc>) -> String {
    let current_year = chrono::Utc::now().year();

    let mut article_file =
        File::open(format!("./site/articles/{}.md", article_metadata.id)).unwrap();
    let mut contents = String::new();
    article_file.read_to_string(&mut contents).unwrap();
    /*     let parser = pulldown_cmark::Parser::new(&contents);

    static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);
    static THEME: LazyLock<Theme> = LazyLock::new(|| {
        let theme_set = ThemeSet::load_defaults();
        theme_set.themes["base16-ocean.dark"].clone()
    });

    let mut sr = SYNTAX_SET.find_syntax_plain_text();
    let mut code = String::new();
    let mut code_block = false;
    let parser = Parser::new(markdown).filter_map(|event| match event {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
            let lang = lang.trim();
            sr = SYNTAX_SET
                .find_syntax_by_token(&lang)
                .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());
            code_block = true;
            None
        }
        Event::End(TagEnd::CodeBlock) => {
            let html = highlighted_html_for_string(&code, &SYNTAX_SET, &sr, &THEME)
                .unwrap_or(code.clone());
            code.clear();
            code_block = false;
            Some(Event::Html(html.into()))
        }

        Event::Text(t) => {
            if code_block {
                code.push_str(&t);
                return None;
            }
            Some(Event::Text(t))
        }
        _ => Some(event),
    });

    let mut html_contents = String::new();
    pulldown_cmark::html::push_html(&mut html_contents, parser); */
    let html_contents = markdown_to_html(&contents);

    let body = html_node::typed::html!(<div class="container-fluid">
    <section id="article-title-section" class="row">
        <section class="col-md-12 text-center">    
                <h1>{text!("{0}", article_metadata.title)}</h1>       
        </section>
            <section class="col-md-12 text-center">
                <h3>Posted: {text!("{0}", article_metadata.created_date.format("%m/%d/%Y"))}</h3>
            </section>
    </section>
        <section id="article-content-section" class="row">
            <section class="col-md-4 offset-md-4">
                <div>
                    {unsafe_text!("{}", html_contents)}
                </div>
            </section>
        </section>
</div>);

    let html = main_layout(body, current_year, None);

    Pretty::from(html).to_string()
}

pub struct ArticleMetadata<Tz: TimeZone> {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created_date: DateTime<Tz>,
    pub modified_date: DateTime<Tz>,
    pub published: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ArticleData {
    #[serde(alias = "Articles")]
    pub articles: Vec<ArticleMetadataString>,
}

#[derive(Serialize, Deserialize)]
pub struct ArticleMetadataString {
    #[serde(alias = "Id")]
    pub id: String,
    #[serde(alias = "Title")]
    pub title: String,
    #[serde(alias = "Description")]
    pub description: String,
    #[serde(alias = "CreatedDate")]
    pub created_date: String,
    #[serde(alias = "ModifiedDate")]
    pub modified_date: String,
    #[serde(alias = "Published")]
    pub published: bool,
}
