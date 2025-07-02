use std::{
    fs::File,
    io::{BufWriter, Read},
    path::PathBuf,
    str::FromStr,
};

use article::{ArticleData, ArticleMetadata, get_article_page};
use chrono::{Datelike, Utc};
use html_node::{pretty::Pretty, text, typed::elements::*, unsafe_text};
use layout::main_layout;
use xml::EmitterConfig;

mod article;
mod layout;
mod markdown;

const CAREER_START: i32 = 2014;
const FULLTIME_CAREER_START: i32 = 2017;

pub fn gen_feed_xml(
    article_metadata: &Vec<ArticleMetadata<Utc>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let now = chrono::Utc::now();
    let current_year = chrono::Utc::now().year();

    let base_url = "https://www.collinsparks.net/";

    let path = "/feed.xml";
    let root_path = "./site";

    let path = append_path(PathBuf::from(root_path), &path);
    let tmp_path = append_path(path.clone(), "_tmp");

    let file = File::create(tmp_path.clone())?;
    let file = BufWriter::new(file);
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(file);

    writer.write(xml::writer::XmlEvent::start_element("rss").attr("version", "2.0"))?;

    writer.write(xml::writer::XmlEvent::start_element("channel"))?;

    writer.write(xml::writer::XmlEvent::start_element("title"))?;
    writer.write(xml::writer::XmlEvent::characters("Collin Sparks Blog"))?;
    writer.write(xml::writer::XmlEvent::end_element())?;

    writer.write(xml::writer::XmlEvent::start_element("link"))?;
    writer.write(xml::writer::XmlEvent::characters(base_url))?;
    writer.write(xml::writer::XmlEvent::end_element())?;

    writer.write(xml::writer::XmlEvent::start_element("description"))?;
    writer.write(xml::writer::XmlEvent::characters(
        "This is the blog of Software Engineer Collin Sparks.",
    ))?;
    writer.write(xml::writer::XmlEvent::end_element())?;

    writer.write(xml::writer::XmlEvent::start_element("copyright"))?;
    writer.write(xml::writer::XmlEvent::characters(
        format!("{} Collin Sparks", current_year).as_str(),
    ))?;
    writer.write(xml::writer::XmlEvent::end_element())?;

    writer.write(xml::writer::XmlEvent::start_element("lastBuildDate"))?;
    writer.write(xml::writer::XmlEvent::characters(now.to_rfc2822().as_str()))?;
    writer.write(xml::writer::XmlEvent::end_element())?;

    for article in article_metadata {
        if article.published {
            writer.write(xml::writer::XmlEvent::start_element("item"))?;

            writer
                .write(xml::writer::XmlEvent::start_element("guid").attr("isPermaLink", "false"))?;
            writer.write(xml::writer::XmlEvent::characters(&article.id))?;
            writer.write(xml::writer::XmlEvent::end_element())?;

            writer.write(xml::writer::XmlEvent::start_element("link"))?;
            writer.write(xml::writer::XmlEvent::characters(
                format!("{}/post/{}", base_url, article.id).as_str(),
            ))?;
            writer.write(xml::writer::XmlEvent::end_element())?;

            writer.write(xml::writer::XmlEvent::start_element("title"))?;
            writer.write(xml::writer::XmlEvent::characters(&article.title))?;
            writer.write(xml::writer::XmlEvent::end_element())?;

            writer.write(xml::writer::XmlEvent::start_element("description"))?;
            writer.write(xml::writer::XmlEvent::characters(&article.description))?;
            writer.write(xml::writer::XmlEvent::end_element())?;

            writer.write(xml::writer::XmlEvent::start_element("pubDate"))?;
            writer.write(xml::writer::XmlEvent::characters(
                article.created_date.to_rfc2822().as_str(),
            ))?;
            writer.write(xml::writer::XmlEvent::end_element())?;

            writer.write(xml::writer::XmlEvent::end_element())?;
        }
    }

    writer.write(xml::writer::XmlEvent::end_element())?;

    writer.write(xml::writer::XmlEvent::end_element())?;

    if std::fs::exists(path.clone())? {
        std::fs::remove_file(path.clone())?;
    }
    std::fs::rename(tmp_path, path)?;

    Ok(())
}

pub fn append_path(path: PathBuf, s: &str) -> PathBuf {
    let mut path = path.into_os_string();
    path.push(s);
    PathBuf::from(path)
}

pub fn get_pages() -> Vec<Page> {
    let mut articles_file = File::open("./site/data/articles.json").unwrap();
    let mut articles_serialized = String::new();
    articles_file
        .read_to_string(&mut articles_serialized)
        .unwrap();

    let mut article_data = serde_json::from_str::<ArticleData>(&articles_serialized).unwrap();
    let mut article_thumbnails = article_data
        .articles
        .drain(..)
        .map(|article| ArticleMetadata::<Utc> {
            id: article.id,
            title: article.title,
            description: article.description,
            created_date: chrono::DateTime::<Utc>::from_str(&article.created_date).unwrap(),
            modified_date: chrono::DateTime::<Utc>::from_str(&article.modified_date).unwrap(),
            published: article.published,
        })
        .collect::<Vec<_>>();

    gen_feed_xml(&article_thumbnails).unwrap();

    article_thumbnails.sort_by(|a, b| b.created_date.cmp(&a.created_date));

    let mut pages = vec![];

    pages.push(Page {
        route: String::from("/"),
        content: get_index_page(),
    });

    pages.push(Page {
        route: String::from("/blog.html"),
        content: get_blog_page(&article_thumbnails),
    });

    pages.push(Page {
        route: String::from("/about.html"),
        content: get_about_page(),
    });

    for article_thumbnail in &article_thumbnails {
        pages.push(Page {
            route: format!("/post/{}.html", article_thumbnail.id),
            content: get_article_page(article_thumbnail),
        });
    }

    pages
}

pub struct Page {
    pub route: String,
    pub content: String,
}

pub fn get_blog_page(article_thumbnails: &Vec<ArticleMetadata<Utc>>) -> String {
    let current_year = chrono::Utc::now().year();

    let article_thumbnails_html = html_node::typed::html!({
        article_thumbnails.iter().filter(|article| {article.published}).map(|article_thumbnail| {
            html_node::typed::html!(<section class="article-thumbnail col-md-12">
                <section class="col-md-12">
                    <h2>{text!("{0}", article_thumbnail.title)}</h2>
                    <p>{text!("{0}", article_thumbnail.description)}</p>
                </section>
                <section class="col-md-12">
                    <ul>
                        <li>{text!("{0}", article_thumbnail.created_date.format("%m.%d.%Y"))}</li>
                    </ul>
                </section>
                <section class="col-md-12">
                    {unsafe_text!("<a class=\"btn\" href=\"post/{0}\">Read More</a>", article_thumbnail.id)}
                </section>
            </section>)
        })
    });

    let body = html_node::typed::html!(<section id="article-thumbnails-section">
    <section class="container">
        <section class="row">
            <section class="col-md-12">
                {article_thumbnails_html}
            </section>
        </section>
    </section>
</section>);

    let html = main_layout(body, current_year, Some(1));

    Pretty::from(html).to_string()
}

pub fn get_index_page() -> String {
    let current_year = chrono::Utc::now().year();

    let body = html_node::typed::html!(
        <script type="module">
        import init from "./wgpu-scenery/pkg/wgpu_scenery.js";
        init().then(() => {
            console.log("WASM Loaded");
        });

    </script>

    <section class="container-fluid">
        <div class="row" style="padding-top:0px;padding-bottom:0px;">
            <section class="col-md-12" style="padding-left:0px;padding-right:0px;">
                <div id="wasm-example" style="height:900px;background: url('/images/nature_scene_demo.jpg') no-repeat center center scroll;
    background-size: cover;"></div>
            </section>
        </div>
    </section>

    <section id="introduction-section" class="container-fluid">
    <div class="row">
        <section class="col-md-8 offset-md-2 title-summary">
            <h2>My name is Collin Sparks and I am a Software Developer.</h2>
            <p>I am based in Georgia and I have worked professionally as a Software Developer for {text!("{0}", current_year - FULLTIME_CAREER_START)} years. My passion for programming has led me to learn a wide variety of frameworks and platforms. You can learn more about me on my <a href="about">about</a> page.</p>
        </section>
    </div>
    </section>);

    let html = main_layout(body, current_year, Some(0));

    Pretty::from(html).to_string()
}

pub fn get_about_page() -> String {
    let current_year = chrono::Utc::now().year();
    let languages = vec![
        "C#",
        "Rust",
        "C++",
        "T-SQL",
        "JavaScript",
        "Objective-C",
        "Java",
        "PHP",
        "HTML",
        "SCSS",
        "CSS",
        "Assembly",
    ];
    let frameworks = vec![
        "ASP.NET Framework",
        "ASP.NET Core",
        "ASP.NET Web Forms",
        "ASP.NET MVC",
        "ASP.NET Web API",
        ".NET Framework",
        ".NET Core",
        "Cocoa Touch",
        "Entity Framework",
        "Entity Framework Core",
        "SQLite",
        "SQL Server",
        "JQuery",
        "Unreal Engine",
        "Blazor",
    ];
    let interests = vec![
        "Web Optimization",
        "Game Engine Architecture",
        "Virtual Reality",
        "Artificial Intelligence",
        "Embedded Systems",
        "Web Assembly",
    ];

    let body = html_node::typed::html!(<div class="container-fluid">
    <section id="about-intro-section" class="row">
        <section class="col-md-6 offset-md-3">
            //@*<img class="img-fluid rounded-circle" src="/images/gradphoto.jpg" />*@
            <section>
                <h1>Collin Sparks</h1>
                <p>I have been a Software Developer for {text!("{}", current_year - CAREER_START)} years and worked full time for {text!("{}", current_year - 2017)} years. Software Development is not only my profession, but my passion as well.</p>
            </section>
        </section>
    </section>
    <section id="about-description-section" class="row title-summary">
        <section class="col-md-8 offset-md-2">
            <h3>About Me</h3>
            <p>
                I have experience professionally with Web Development as well as Mobile Development. In my spare time I enjoy working on a wide variety of different side projects to keep my skills sharp.
                Game Development for example allows me to pit myself against challenges I do not find myself faced with in my professional work.
            </p>
        </section>
    </section>
</div>

<div id="technology-section" class="container-fluid">
    <div class="row">
        <section class="col-md-8 offset-md-2 title-summary">
            <h3>Technologies</h3>
            <p>Years of experience have taught me the following technologies well:</p>
        </section>
    </div>
    <div class="row">
        <div class="col-md-5 offset-md-4">
            <div class="container-fluid">
                <div class="row row-cols-3 item-list-section-2">
                    <div class="col"></div>
                    <div class="col"><h3>Languages</h3></div>
                    <div class="col"></div>
                    {
                        languages.into_iter().map(|language| html_node::typed::html!(<div class="col"><span>{text!("{language}")}</span></div>))
                    }
                </div>
            </div>
        </div>

        <div class="col-md-5 offset-md-4">
            <div class="container-fluid">
                <div class="row row-cols-3 item-list-section-2">
                    <div class="col"></div>
                    <div class="col"><h3>Frameworks</h3></div>
                    <div class="col"></div>
                    {
                        frameworks.into_iter().map(|framework| html_node::typed::html!(<div class="col"><span>{text!("{framework}")}</span></div>))
                    }
                </div>
            </div>
        </div>
    </div>
</div>

<div id="interest-section" class="container-fluid">
    <div class="row">
        <section class="col-md-8 offset-md-2 title-summary">
            <h3>Interests</h3>
            <p>When I do have spare time I spend most of it trying to learn something I have not had the opportunity to learn at work, or in school.</p>
        </section>
        <div class="col-md-5 offset-md-4">
            <div class="container-fluid">
                <div class="row row-cols-3 item-list-section-2">
                    {
                        interests.into_iter().map(|interest| html_node::typed::html!(<div class="col"><span>{text!("{interest}")}</span></div>))
                    }
                </div>
            </div>
        </div>
    </div>
</div>);

    let html = main_layout(body, current_year, Some(2));

    Pretty::from(html).to_string()
}
