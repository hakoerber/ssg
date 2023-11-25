use std::path::Path;

use super::data;
use super::icon;
use super::icon::Icon;
use super::{frame, render};
use super::{FileType, Page};

use maud::html;

pub trait Render {
    fn render(output_base_path: &Path, input_path: &Path) -> Vec<Page>;
}

pub struct LandingPage;

impl Render for LandingPage {
    fn render(output_base_path: &Path, input_path: &Path) -> Vec<Page> {
        let (dir, name) = ("", "index.html");

        let path = output_base_path.join(dir).join(name);
        let page = html!(
            div #landing {
                div id="introduction" {
                    h1 { "Hi!"}
                    p {
                        "Hello, welcome to my homepage! Here, you will find some articles
                        (mostly tech), some info about myself and whatever else I am thinking of."
                    }
                }
                img src="assets/profile.jpg" {}
            }
        );

        render::render_into(frame(data::FULLNAME, page, input_path), &path);
        vec![Page {
            path: Path::new(dir).join(name).to_str().unwrap().to_owned(),
            filetype: FileType::Html,
        }]
    }
}

pub struct SkillsPage;

impl Render for SkillsPage {
    fn render(output_base_path: &Path, input_path: &Path) -> Vec<Page> {
        let (dir, name) = ("skills", "index.html");

        let directory = output_base_path.join(dir);
        std::fs::create_dir_all(&directory).unwrap();

        let path = directory.join(name);

        let page = html!(
            main #skills {
                article #focus-areas {
                    h1 { "Focus Areas" }
                    hr;
                    div {
                        div .column {
                            section {
                                h1 {
                                    img src=(icon!("CloudDownload", input_path).output_path()) {}
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
                                    img src=(icon!("MagnifyingGlass", input_path).output_path()) {}
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
                                    img src=(icon!("Network", input_path).output_path()) {}
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
                                    img src=(icon!("Shield", input_path).output_path()) {}
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
                                    img src=(icon!("Gears", input_path).output_path()) {}
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
                        @for category in data::tech_categories(&input_path) {
                            section .block {
                                div .name {
                                    h1 { (category.name) }
                                }
                                div .techlist {
                                    div {
                                        @for tech in category.technologies {
                                            div .tech data-tech-level={(tech.level)} {
                                                img src=(tech.icon.output_path()) {}
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
                        @for certification in data::certifications() {
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

        render::render_into(frame(data::FULLNAME, page, input_path), &path);
        vec![Page {
            path: Path::new(dir).join(name).to_str().unwrap().to_owned(),
            filetype: FileType::Html,
        }]
    }
}
