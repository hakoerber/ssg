use serde::{Deserialize, Serialize};
use std::env;
use std::io::Write;
use std::path::{Path, PathBuf};

use comrak::plugins::syntect::SyntectAdapter;
use comrak::{markdown_to_html_with_plugins, Options, Plugins};
use maud::{html, Markup};

mod data;
mod fs;
mod icon;
mod pages;
mod render;

use pages::Render;

static mut SEEN_ICONS: Vec<&'static str> = vec![];

#[derive(Debug, Serialize)]
enum FileType {
    Html,
    Svg,
    Css,
    Jpg,
    Png,
}

impl FileType {
    fn detect(path: &str) -> Result<Self, String> {
        let path = Path::new(path);
        let ext = path.extension().unwrap().to_str().unwrap();
        Ok(match ext {
            "jpg" => FileType::Jpg,
            "svg" => FileType::Svg,
            "png" => FileType::Png,
            _ => return Err(format!("unknown extension: {ext}")),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct Page {
    path: String,
    filetype: FileType,
}

#[derive(Debug, Serialize)]
struct Manifest {
    content_directory: PathBuf,
    pages: Vec<Page>,
}

#[derive(Deserialize, Clone)]
struct Tag(String);

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
struct Frontmatter {
    title: String,
    #[serde(with = "time::serde::rfc3339", rename = "date")]
    timestamp: time::OffsetDateTime,
    summary: String,
    #[allow(dead_code)]
    tags: Vec<Tag>,
}

fn frame(title: &str, inner: Markup, input_path: &Path) -> Markup {
    let year = time::OffsetDateTime::now_utc().year();

    struct Page {
        name: &'static str,
        link: String,
    }

    let pages = [
        Page {
            name: "Blog",
            link: "/blog/index.html".into(),
        },
        Page {
            name: "Skills",
            link: "/skills/index.html".into(),
        },
        Page {
            name: "Projects",
            link: "/projects/index.html".into(),
        },
        Page {
            name: "About Me",
            link: "/about/index.html".into(),
        },
    ];

    let output = html!(
        (maud::DOCTYPE)
        html {
            head {
                title { (title) }
                link rel="stylesheet" href="/reset.css" {}
                link rel="stylesheet" href="/style.css" {}
                link rel="icon" href="/favicon.svg"
                script src="https://unpkg.com/htmx.org@1.9.9" {}
                meta charset="utf-8" {}
                meta name="viewport" content="width=device-width, initial-scale=1.0" {}
            }
            body hx-boost="true" {
                header {
                    nav aria-label="main navigation" {
                        a .title href="/index.html" {
                            (data::FULLNAME)
                        }
                        div .links {
                            @for page in &pages {
                                a
                                    .link
                                    href=(page.link)
                                    title=(page.name)
                                {
                                    (page.name)
                                }
                            }
                        }
                    }
                }

                (inner)

                footer {
                    div .socials {
                        @for social in &data::socials(&input_path) {
                            a
                                href=(social.link)
                                title=(social.description.unwrap_or(&format!("Me on {}", social.name)))
                                target="_blank" rel="noopener noreferrer"
                            {
                                img src=(social.icon.output_path()) {}
                            }
                        }
                    }

                    div .badges {
                        @for certification in data::certifications() {
                            a
                                href=(certification.link)
                                title=(certification.title)
                                target="_blank" rel="noopener noreferrer"
                            {
                                figure {
                                    img src=(certification.image) {}
                                }
                            }
                        }
                    }
                    div .copyright {
                        span { (format!("© {}, {year}", data::FULLNAME)) }
                    }

                }
            }
        }
    );

    output
}

struct Blogpost {
    frontmatter: Frontmatter,
    html_filename: String,
}

fn render_blogposts(output_base_path: &Path, input_path: &Path) -> Vec<Page> {
    let (dir, index) = ("blog", "index.html");

    let mut pages = vec![];
    let adapter = SyntectAdapter::new("InspiredGitHub");

    let out = output_base_path.join(dir);
    std::fs::create_dir_all(out.as_path()).unwrap();

    let mut blog_posts: Vec<Blogpost> = vec![];

    for entry in std::fs::read_dir(input_path.join("blog")).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if !entry.file_type().unwrap().is_file() {
            panic!("not a file: {path:?}");
        }

        if path.extension().unwrap() != "md" {
            panic!("unknown file type found: {path:?}");
        }

        let mut plugins = Plugins::default();

        plugins.render.codefence_syntax_highlighter = Some(&adapter);

        let file = std::fs::read_to_string(&path).unwrap();
        let mut file = file.lines();

        let frontmatter = file
            .by_ref()
            .take_while(|line| *line != "---")
            .map(|l| format!("{l}\n"))
            .collect::<String>();

        let frontmatter: Frontmatter = toml::from_str(&frontmatter).unwrap();

        let rest = file.map(|l| format!("{l}\n")).collect::<String>();

        let md_options = Options {
            render: {
                let mut builder = comrak::RenderOptionsBuilder::default();
                builder.github_pre_lang(true);
                builder.build().unwrap()
            },
            extension: {
                let mut builder = comrak::ExtensionOptionsBuilder::default();
                builder.header_ids(None);
                builder.table(true);
                builder.build().unwrap()
            },
            ..Default::default()
        };

        let output = markdown_to_html_with_plugins(&rest, &md_options, &plugins);

        let inner = html!(
            article #blogpost {
                div .header {
                    h1 { (frontmatter.title) }
                    div .meta {
                        p .summary { (maud::PreEscaped(&frontmatter.summary)) }
                        p .date { (frontmatter.timestamp.date()) }
                    }
                }
                div .content {
                    (maud::PreEscaped(output))
                }
            }
        );

        let output = frame(&frontmatter.title, inner, &input_path).into_string();

        let mut path = path.clone();
        assert!(path.set_extension("html"));
        let html_filename = path.file_name().unwrap().to_str().unwrap();

        let out_path = out.as_path().join(html_filename);
        let mut handle = std::fs::File::create(&out_path).unwrap();
        handle.write_all(output.as_bytes()).unwrap();

        pages.push(Page {
            path: Path::new(dir)
                .join(html_filename)
                .to_str()
                .unwrap()
                .to_owned(),
            filetype: FileType::Html,
        });

        blog_posts.push(Blogpost {
            frontmatter,
            html_filename: html_filename.to_string(),
        });
    }

    blog_posts.sort_by_key(|post| post.frontmatter.timestamp);
    blog_posts.reverse();

    let inner = html!(
        div .postlist {
            table {
                tbody {
                    @for blog_post in &blog_posts {
                        tr {
                            td {
                                a href=(Path::new("/blog").join(blog_post.html_filename.clone()).to_str().unwrap()) {
                                    (blog_post.frontmatter.title)
                                }
                            }
                            td {
                                (blog_post.frontmatter.timestamp.date())
                            }
                        }
                    }
                }
            }
        }
    );

    let output = frame("Blog posts", inner, input_path);

    let output_path = &out.as_path().join(index);
    render::render_into(output, &output_path);

    pages.push(Page {
        path: Path::new(dir).join(index).to_str().unwrap().to_owned(),
        filetype: FileType::Html,
    });

    pages
}

#[derive(PartialEq, Eq)]
enum CheckMode {
    Relaxed,
    Strict,
}

impl TryFrom<String> for CheckMode {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(match value.as_str() {
            "relaxed" => Self::Relaxed,
            "strict" => Self::Strict,
            _ => return Err(format!("unknown checkmode value {value}")),
        })
    }
}

fn main() {
    let mut pages: Vec<Page> = vec![];
    let (check_mode, output_base_path): (CheckMode, PathBuf) = {
        let mut args = env::args().skip(1);
        (
            args.next().unwrap().try_into().unwrap(),
            PathBuf::from(args.next().unwrap()),
        )
    };

    assert!(output_base_path.is_absolute());

    let input_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .canonicalize()
        .unwrap();

    assert!(input_path.exists());

    let build_directory = "build";

    let rendered_output_directory = output_base_path.join(build_directory);

    std::fs::create_dir_all(output_base_path.as_path()).unwrap();
    std::fs::create_dir_all(rendered_output_directory.as_path()).unwrap();

    pages.append(&mut render_blogposts(
        &rendered_output_directory,
        &input_path,
    ));

    pages.append(&mut pages::LandingPage::render(
        &rendered_output_directory,
        &input_path,
    ));
    pages.append(&mut pages::SkillsPage::render(
        &rendered_output_directory,
        &input_path,
    ));
    pages.append(&mut pages::ProjectsPage::render(
        &rendered_output_directory,
        &input_path,
    ));
    pages.append(&mut pages::AboutPage::render(
        &rendered_output_directory,
        &input_path,
    ));

    let icons = icon::IconsUnverified::verify_all(
        if check_mode == CheckMode::Relaxed {
            icon::UnusedIconFiles::Allow
        } else {
            icon::UnusedIconFiles::Deny
        },
        &input_path,
    );

    fn copy(rendered_output_directory: &Path, path: &'static str, input_path: &Path) -> Page {
        std::fs::copy(
            input_path.join("static").join(path),
            rendered_output_directory.join(path),
        )
        .unwrap();
        Page {
            path: path.to_owned(),
            filetype: FileType::Css,
        }
    }

    pages.push(copy(&rendered_output_directory, "reset.css", &input_path));
    pages.push(copy(&rendered_output_directory, "style.css", &input_path));
    pages.push(copy(&rendered_output_directory, "favicon.svg", &input_path));

    pages.append(
        &mut fs::copy_dir_all(
            input_path.join("static/assets"),
            &rendered_output_directory,
            Path::new("assets"),
        )
        .unwrap()
        .into_iter()
        .map(|path| Page {
            filetype: FileType::detect(&path).unwrap(),
            path,
        })
        .collect(),
    );
    pages.append(&mut icons.copy_all(&rendered_output_directory, &input_path));

    let manifest = Manifest {
        pages,
        content_directory: Path::new(build_directory).to_owned(),
    };

    let mut handle = std::fs::File::create(output_base_path.join("manifest.json")).unwrap();
    handle
        .write_all(serde_json::to_string(&manifest).unwrap().as_bytes())
        .unwrap();
}
