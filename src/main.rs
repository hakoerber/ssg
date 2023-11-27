use serde::Deserialize;
use std::env;
use std::fmt;
use std::io::Write;
use std::path::{Path, PathBuf};

use comrak::plugins::syntect::SyntectAdapter;
use comrak::{markdown_to_html_with_plugins, Options, Plugins};
use maud::{html, Markup};

mod fs;
mod icon;
mod render;

use icon::Icon;

struct Certification {
    link: &'static str,
    title: &'static str,
    image: String,
}

fn certifications(output_base_path: &Path) -> Vec<Certification> {
    vec![Certification {
        link: "https://www.credly.com/badges/870a6345-ed4e-416e-9c46-c9af9c6d2c77/public_url",
        title: "AWS Certified Solutions Architect – Associate",
        image: output_base_path
            .join("assets/badges/aws-certified-solutions-architect-associate.png")
            .to_str()
            .unwrap()
            .to_owned(),
    }]
}

struct Social {
    name: &'static str,
    link: String,
    icon: Box<dyn icon::Icon>,
    description: Option<&'static str>,
}

fn socials(output_base_path: &Path) -> Vec<Social> {
    vec![
        Social {
            name: "Github",
            link: "https://github.com/hakoerber".into(),
            icon: icon!("Github"),
            description: None,
        },
        Social {
            name: "Linkedin",
            link: "https://www.linkedin.com/in/hannes-koerber".into(),
            icon: icon!("Linkedin"),
            description: None,
        },
        Social {
            name: "Keybase",
            link: "https://keybase.io/hakoerber".into(),
            icon: icon!("Keybase"),
            description: None,
        },
        Social {
            name: "E-Mail",
            link: "mailto:hannes.koerber@gmail.com".into(),
            icon: icon!("Email"),
            description: Some("Send me an e-mail"),
        },
        Social {
            name: "RSS",
            link: output_base_path
                .join("rss.xml")
                .to_str()
                .unwrap()
                .to_owned(),
            icon: icon!("Rss"),
            description: Some("Follow my blog on RSS"),
        },
    ]
}

enum TechLevel {
    Pro,
    Normal,
}

impl fmt::Display for TechLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Pro => "pro",
                Self::Normal => "normal",
            }
        )
    }
}

struct Technology {
    name: &'static str,
    level: TechLevel,
    icon: Box<dyn icon::Icon>,
}

struct TechCategory {
    name: &'static str,
    technologies: Vec<Technology>,
}

fn tech_categories() -> Vec<TechCategory> {
    vec![
        TechCategory {
            name: "Containerization",
            technologies: vec![
                Technology {
                    name: "Kubernetes",
                    level: TechLevel::Pro,
                    icon: icon!("Kubernetes"),
                },
                Technology {
                    name: "Docker",
                    level: TechLevel::Pro,
                    icon: icon!("Docker"),
                },
                Technology {
                    name: "cri-o",
                    level: TechLevel::Normal,
                    icon: icon!("CriO"),
                },
                Technology {
                    name: "Containerd",
                    level: TechLevel::Normal,
                    icon: icon!("Containerd"),
                },
                Technology {
                    name: "OCI",
                    level: TechLevel::Normal,
                    icon: icon!("Oci"),
                },
                Technology {
                    name: "Rancher",
                    level: TechLevel::Normal,
                    icon: icon!("Rancher"),
                },
            ],
        },
        TechCategory {
            name: "Databases",
            technologies: vec![
                Technology {
                    name: "PostgreSQL",
                    level: TechLevel::Pro,
                    icon: icon!("Postgresql"),
                },
                Technology {
                    name: "ElasticSearch",
                    level: TechLevel::Pro,
                    icon: icon!("Elasticsearch"),
                },
                Technology {
                    name: "MySQL",
                    level: TechLevel::Pro,
                    icon: icon!("Mysql"),
                },
                Technology {
                    name: "Redis",
                    level: TechLevel::Normal,
                    icon: icon!("Redis"),
                },
                Technology {
                    name: "InfluxDB",
                    level: TechLevel::Normal,
                    icon: icon!("Influx"),
                },
                Technology {
                    name: "SQLite",
                    level: TechLevel::Normal,
                    icon: icon!("Sqlite"),
                },
            ],
        },
        TechCategory {
            name: "Configuration Management",
            technologies: vec![
                Technology {
                    name: "Terraform",
                    level: TechLevel::Pro,
                    icon: icon!("Terraform"),
                },
                Technology {
                    name: "Ansible",
                    level: TechLevel::Pro,
                    icon: icon!("Ansible"),
                },
                Technology {
                    name: "Pulumi",
                    level: TechLevel::Pro,
                    icon: icon!("Pulumi"),
                },
                Technology {
                    name: "Packer",
                    level: TechLevel::Normal,
                    icon: icon!("Packer"),
                },
                Technology {
                    name: "Puppet",
                    level: TechLevel::Normal,
                    icon: icon!("Puppet"),
                },
                Technology {
                    name: "SaltStack",
                    level: TechLevel::Normal,
                    icon: icon!("Saltstack"),
                },
            ],
        },
        TechCategory {
            name: "Web Development",
            technologies: vec![
                Technology {
                    name: "HTML",
                    level: TechLevel::Pro,
                    icon: icon!("Html5"),
                },
                Technology {
                    name: "CCS",
                    level: TechLevel::Pro,
                    icon: icon!("Css"),
                },
                Technology {
                    name: "JavaScript",
                    level: TechLevel::Normal,
                    icon: icon!("Javascript"),
                },
                Technology {
                    name: "Flask",
                    level: TechLevel::Normal,
                    icon: icon!("Flask"),
                },
                Technology {
                    name: "Svelte",
                    level: TechLevel::Normal,
                    icon: icon!("Svelte"),
                },
                Technology {
                    name: "ReactJS",
                    level: TechLevel::Normal,
                    icon: icon!("Reactjs"),
                },
            ],
        },
        TechCategory {
            name: "Programming",
            technologies: vec![
                Technology {
                    name: "Python",
                    level: TechLevel::Pro,
                    icon: icon!("Python"),
                },
                Technology {
                    name: "Rust",
                    level: TechLevel::Pro,
                    icon: icon!("Rust"),
                },
                Technology {
                    name: "Go",
                    level: TechLevel::Pro,
                    icon: icon!("Go"),
                },
                Technology {
                    name: "TypeScript",
                    level: TechLevel::Normal,
                    icon: icon!("Typescript"),
                },
                Technology {
                    name: "Bash",
                    level: TechLevel::Normal,
                    icon: icon!("Bash"),
                },
                Technology {
                    name: "C",
                    level: TechLevel::Normal,
                    icon: icon!("C"),
                },
            ],
        },
        TechCategory {
            name: "Observability",
            technologies: vec![
                Technology {
                    name: "Prometheus",
                    level: TechLevel::Pro,
                    icon: icon!("Prometheus"),
                },
                Technology {
                    name: "Grafana",
                    level: TechLevel::Pro,
                    icon: icon!("Grafana"),
                },
                Technology {
                    name: "Kibana",
                    level: TechLevel::Normal,
                    icon: icon!("Kibana"),
                },
                Technology {
                    name: "OpsGenie",
                    level: TechLevel::Normal,
                    icon: icon!("Opsgenie"),
                },
                Technology {
                    name: "OpenTelemetry",
                    level: TechLevel::Normal,
                    icon: icon!("Opentelemetry"),
                },
                Technology {
                    name: "Jaeger",
                    level: TechLevel::Normal,
                    icon: icon!("Jaeger"),
                },
            ],
        },
        TechCategory {
            name: "Development",
            technologies: vec![
                Technology {
                    name: "Git",
                    level: TechLevel::Pro,
                    icon: icon!("Git"),
                },
                Technology {
                    name: "Neovim",
                    level: TechLevel::Pro,
                    icon: icon!("Neovim"),
                },
                Technology {
                    name: "GitLab",
                    level: TechLevel::Normal,
                    icon: icon!("Gitlab"),
                },
                Technology {
                    name: "GitHub",
                    level: TechLevel::Normal,
                    icon: icon!("Github"),
                },
                Technology {
                    name: "OpenAPI",
                    level: TechLevel::Normal,
                    icon: icon!("Swagger"),
                },
                Technology {
                    name: "Jira",
                    level: TechLevel::Normal,
                    icon: icon!("Jira"),
                },
            ],
        },
        TechCategory {
            name: "Automation",
            technologies: vec![
                Technology {
                    name: "Drone",
                    level: TechLevel::Pro,
                    icon: icon!("Drone"),
                },
                Technology {
                    name: "GitLab CI",
                    level: TechLevel::Pro,
                    icon: icon!("Gitlab"),
                },
                Technology {
                    name: "Jenkins",
                    level: TechLevel::Normal,
                    icon: icon!("Jenkins"),
                },
            ],
        },
        TechCategory {
            name: "Cloud Providers",
            technologies: vec![
                Technology {
                    name: "AWS",
                    level: TechLevel::Pro,
                    icon: icon!("Aws"),
                },
                Technology {
                    name: "DigitalOcean",
                    level: TechLevel::Normal,
                    icon: icon!("Digitalocean"),
                },
                Technology {
                    name: "Hetzner",
                    level: TechLevel::Normal,
                    icon: icon!("Hetzner"),
                },
            ],
        },
        TechCategory {
            name: "Web Servers",
            technologies: vec![
                Technology {
                    name: "Nginx",
                    level: TechLevel::Pro,
                    icon: icon!("Nginx"),
                },
                Technology {
                    name: "Apache",
                    level: TechLevel::Normal,
                    icon: icon!("Apache"),
                },
                Technology {
                    name: "HAProxy",
                    level: TechLevel::Normal,
                    icon: icon!("Haproxy"),
                },
                Technology {
                    name: "OpenResty",
                    level: TechLevel::Normal,
                    icon: icon!("Openresty"),
                },
            ],
        },
        TechCategory {
            name: "Operating Systems",
            technologies: vec![
                Technology {
                    name: "CentOS",
                    level: TechLevel::Pro,
                    icon: icon!("Centos"),
                },
                Technology {
                    name: "Debian",
                    level: TechLevel::Pro,
                    icon: icon!("Debian"),
                },
                Technology {
                    name: "Arch Linux",
                    level: TechLevel::Pro,
                    icon: icon!("ArchLinux"),
                },
                Technology {
                    name: "Ubuntu",
                    level: TechLevel::Normal,
                    icon: icon!("Ubuntu"),
                },
                Technology {
                    name: "Fedora",
                    level: TechLevel::Normal,
                    icon: icon!("Fedora"),
                },
                Technology {
                    name: "FreeBSD",
                    level: TechLevel::Normal,
                    icon: icon!("Freebsd"),
                },
            ],
        },
        TechCategory {
            name: "Security",
            technologies: vec![
                Technology {
                    name: "Keycloak",
                    level: TechLevel::Pro,
                    icon: icon!("Keycloak"),
                },
                Technology {
                    name: "OpenID Connect",
                    level: TechLevel::Pro,
                    icon: icon!("OpenidConnect"),
                },
                Technology {
                    name: "GnuPG",
                    level: TechLevel::Normal,
                    icon: icon!("Gnupg"),
                },
                Technology {
                    name: "Let's Encrypt",
                    level: TechLevel::Normal,
                    icon: icon!("Letsencrypt"),
                },
                Technology {
                    name: "Wireshark",
                    level: TechLevel::Normal,
                    icon: icon!("Wireshark"),
                },
                Technology {
                    name: "OpenVPN",
                    level: TechLevel::Normal,
                    icon: icon!("Openvpn"),
                },
            ],
        },
        TechCategory {
            name: "Virtualization",
            technologies: vec![
                Technology {
                    name: "Libvirt / KVM",
                    level: TechLevel::Pro,
                    icon: icon!("Libvirt"),
                },
                Technology {
                    name: "Vagrant",
                    level: TechLevel::Pro,
                    icon: icon!("Vagrant"),
                },
                Technology {
                    name: "Qemu",
                    level: TechLevel::Normal,
                    icon: icon!("Qemu"),
                },
            ],
        },
        TechCategory {
            name: "Storage",
            technologies: vec![
                Technology {
                    name: "Ceph",
                    level: TechLevel::Pro,
                    icon: icon!("Ceph"),
                },
                Technology {
                    name: "ZFS",
                    level: TechLevel::Normal,
                    icon: icon!("Openzfs"),
                },
            ],
        },
    ]
}

const FULLNAME: &str = "Hannes Körber";

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

fn frame(output_base_path: &Path, title: &str, inner: Markup) -> Markup {
    let year = time::OffsetDateTime::now_utc().year();

    struct Page {
        name: &'static str,
        link: String,
    }

    let pages = [
        Page {
            name: "Blog",
            link: output_base_path
                .join("blog/index.html")
                .to_str()
                .unwrap()
                .to_owned(),
        },
        Page {
            name: "Skills",
            link: output_base_path
                .join("skills/index.html")
                .to_str()
                .unwrap()
                .to_owned(),
        },
        Page {
            name: "Projects",
            link: "projects".into(),
        },
        Page {
            name: "About Me",
            link: "about".into(),
        },
    ];

    let output = html!(
        (maud::DOCTYPE)
        html {
            head {
                title { (title) }
                link rel="stylesheet" href=(output_base_path.join("reset.css").to_str().unwrap()) {}
                link rel="stylesheet" href=(output_base_path.join("style.css").to_str().unwrap()) {}
                meta name="viewport" content="width=device-width, initial-scale=1.0" {}
            }
            body {
                header {
                    nav aria-label="main navigation" {
                        a .title href=(output_base_path.join("index.html").to_str().unwrap()) {
                            (FULLNAME)
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
                        @for social in &socials(output_base_path) {
                            a
                                href=(social.link)
                                title=(social.description.unwrap_or(&format!("Me on {}", social.name)))
                            {
                                img src=(social.icon.path(output_base_path)) {}
                            }
                        }
                    }

                    div .badges {
                        @for certification in certifications(output_base_path) {
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
                        span { (format!("© {FULLNAME}, {year}")) }
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

fn render_blogposts(output_base_path: &Path) {
    let adapter = SyntectAdapter::new("InspiredGitHub");

    let out = output_base_path.join("blog");
    std::fs::create_dir_all(out.as_path()).unwrap();

    let mut blog_posts: Vec<Blogpost> = vec![];

    for entry in std::fs::read_dir("./blog").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        println!("found file {path:?}");

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

        let output = frame(output_base_path, &frontmatter.title, inner).into_string();

        let mut path = path.clone();
        assert!(path.set_extension("html"));
        let html_filename = path.file_name().unwrap().to_str().unwrap();

        let mut handle = std::fs::File::create(out.as_path().join(html_filename)).unwrap();
        handle.write_all(output.as_bytes()).unwrap();

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
                    @for blog_post in blog_posts {
                        tr {
                            td {
                                a href=(output_base_path.join("blog").join(blog_post.html_filename).to_str().unwrap()) {
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

    let output = frame(output_base_path, "Blog posts", inner);

    render::render_into(output, &out.as_path().join("index.html"));
}

fn render_landing_page(output_base_path: &Path) {
    let path = output_base_path.join("index.html");
    let page = html!(
        div #landing {
            div id="introduction" {
                h1 { "Hi!"}
                p {
                    "Hello, welcome to my homepage! Here, you will find some articles
                    (mostly tech), some info about myself and whatever else I am thinking of."
                }
            }
            img src=(output_base_path.join("assets/profile.jpg").to_str().unwrap()) {}
        }
    );

    render::render_into(frame(output_base_path, FULLNAME, page), &path);
}

fn render_skills_page(output_base_path: &Path) {
    let directory = output_base_path.join("skills");
    std::fs::create_dir_all(&directory).unwrap();

    let path = directory.join("index.html");

    let page = html!(
        main #skills {
            article #focus-areas {
                h1 { "Focus Areas" }
                hr;
                div {
                    div .column {
                        section {
                            h1 {
                                img src=(icon!("CloudDownload").path(output_base_path)) {}
                                span { "Cloud & Migrations" }
                            }

                            p .slogan {
                                "I help you get your infrastructure ready for the cloud"
                            }
                            div {
                                p {
                                    "I have worked a lot of time with bare-metal, on-premises
                                    infrastructure. It became obvious that cloud services have 
                                    a huge amount of benefits, but requires a lot of experience 
                                    to make sure to avoid the drawbacks like cost traps or vendor lock-in."
                                }
                                p {
                                    "I am a big proponent of Infrastructure-as-Code (IaC) and immutable
                                    infrastructure, as far as it can be sensibily achieved. Having your whole
                                    infrastructure in a git repository and being able to recreate, clone, 
                                    scale or change all assets with a single command is, for me, the most 
                                    important benefit of a cloud-native infrastructure."
                                }
                                p {
                                    "The most difficult aspect of a cloud migration is to keep the drawbacks
                                    in check while still leveraging the benefits. Keeping an eye on costs 
                                    and potential vendor lock-in is paramount."
                                }
                            }
                        }

                        section {
                            h1 {
                                img src=(icon!("MagnifyingGlass").path(output_base_path)) {}
                                span { "Monitoring & Alerting" }
                            }
                            p .slogan {
                                "I help you get insights into your infrastructure"
                            }
                            div {
                                p {
                                    "I really like to have detailed, comprehensive monitoring for everything
                                    in a system. This goes through the whole stack, from the infrastructure 
                                    parts to the application."
                                }
                                p {
                                    "Same goes for logging. With an effective combination of metrics and
                                    events nearly every problem can be traced back to its root cause."
                                }
                                p {
                                    "I did 24/7 on-call duty rotations, so I have some on-hands experience
                                    with alerting and know what to improve and optimize."
                                }
                            }
                        }
                    }

                    div .column {
                        section {
                            h1 {
                                img src=(icon!("Network").path(output_base_path)) {}
                                span { "DevOps Architecture" }
                            }
                            p .slogan {
                                "I help you build reliable, scalable services"
                            }
                            div {
                                p {
                                    "In the past, I have worked with many different kind of applications.
                                    From big monoliths to small, stateless microservices. There are a lot 
                                    of different approaches to architecture and infrastructure, and none 
                                    is strictly better than the other."
                                }
                                p {
                                    "Instead of focussing on a single appraoch (e.g. microservices), I
                                    prefer to adapt the solution to the requirements. Over time, I came 
                                    to recognize the following values in good architecture:"
                                }
                                ul {
                                    li { "Composability"}
                                    li { "Clear separation of concerns"}
                                    li { "Explicit and confined state"}
                                    li { "API-driven infrastructure"}
                                }
                                p {
                                    "\"DevOps\" is one of the most misunderstood concepts that currently
                                    exists in the IT industry. Nevertheless, I am convinced that actual 
                                    DevOps is the most effective way to build software. By having a tight 
                                    coupling between application code and infrastructure, a whole family 
                                    of potential problems are eliminted before they even appear."
                                }
                                p {
                                    "Because I have experience both with the infrastructure and the
                                    application side I am able to build full-stack application that 
                                    adhere to this DevOps mentality and enable architectures that 
                                    would not even be possible in a traditional approach."
                                }
                            }
                        }
                    }

                    div .column {
                        section {
                            h1 {
                                img src=(icon!("Shield").path(output_base_path)) {}
                                span { "Security" }
                            }
                            p .slogan {
                                "I help you ensure the security or your data and applications"
                            }
                            div {
                                p {
                                    "To me, information security is one of the most critical aspects
                                    in today's IT landscape. Many recent changes and technologies made
                                    traditional security approaches obsolete, or even dangerous to apply."
                                }
                                p {
                                    "I am really fond of of Google's BeyondCorp zero-trust security
                                    concept, leveraging protocols like OAuth and OpenID Connect. In 
                                    the end, security is not all-or-nothing, it is a spectrum and 
                                    has many different aspects, from code to humans."
                                }
                                p {
                                    "Also, security is not something you can just tack on existing applications
                                    after the fact. Security engineering has to be an integrated part of your 
                                    application development, processes and, effectively, the whole company. You 
                                    are never really \"done\" with security."
                                }
                            }
                        }
                        section {
                            h1 {
                                img src=(icon!("Gears").path(output_base_path)) {}
                                span { "Automation" }
                            }
                            p .slogan {
                                "I help you automate as much as possible"
                            }
                            div {
                                p {
                                    "Everything done manually will be done wrong eventually. But
                                    computers are very good at doing the exact same tasks over and 
                                    over and over again, so let's delegate as much as possible to them!"
                                }
                                p {
                                    "Using orchestration tooling like Ansible enables us to track all
                                    tasks in version control, review them, and execute them automatically.
                                    Combined with CI tools like Drone or automation suites like 
                                    Rundeck makes it possible to have all changes and regular tasks 
                                    done automatically."
                                }
                                p {
                                    "My philosophy is to never do a tasks twice manually. Before you do it a
                                    second time, automate it away."
                                }
                            }
                        }
                    }
                }
            }

            article #technologies {
                h1 { "Technologies" }
                hr;
                div {
                    @for category in tech_categories() {
                        section .block {
                            div .name {
                                h1 { (category.name) }
                            }
                            div .techlist {
                                div {
                                    @for tech in category.technologies {
                                        div .tech data-tech-level={(tech.level)} {
                                            img src=(tech.icon.path(output_base_path)) {}
                                            span { (tech.name) }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            article #certifications {
                h1 { "Certifications" }
                hr;
                ul {
                    @for certification in certifications(output_base_path) {
                        li {
                            a href=(certification.link) title=(certification.title) target="_blank" rel="noopener noreferrer" {
                                img src=(certification.image) {}
                                h1 { (certification.title) }
                            }
                        }
                    }
                }
            }
        }
    );

    render::render_into(frame(output_base_path, FULLNAME, page), &path);
}

fn main() {
    let output_base_path = {
        let mut args = env::args().skip(1);
        PathBuf::from(args.next().unwrap())
    };

    assert!(output_base_path.is_absolute());

    std::fs::create_dir_all(output_base_path.as_path()).unwrap();

    render_blogposts(&output_base_path);

    render_landing_page(&output_base_path);
    render_skills_page(&output_base_path);

    let icons = icon::IconsUnverified::verify_all(if cfg!(debug_assertions) {
        icon::UnusedIconFiles::Allow
    } else {
        icon::UnusedIconFiles::Deny
    });

    std::fs::copy("./static/reset.css", output_base_path.join("reset.css")).unwrap();
    fs::copy_dir_all("./static/assets", output_base_path.join("assets")).unwrap();
    icons.copy_all(&output_base_path);
    std::fs::copy("./static/style.css", output_base_path.join("style.css")).unwrap();
}
