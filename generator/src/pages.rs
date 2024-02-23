use std::path::Path;

use super::data;
use super::icon;
use super::icon::Icon;
use super::{frame, render};
use super::{FileType, Page};

use maud::{html, PreEscaped};

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

pub struct ProjectsPage;

impl Render for ProjectsPage {
    fn render(output_base_path: &Path, input_path: &Path) -> Vec<Page> {
        let (dir, name) = ("projects", "index.html");

        let directory = output_base_path.join(dir);
        std::fs::create_dir_all(&directory).unwrap();

        let path = directory.join(name);

        let page = html!(
            main #projects {
                div #ownprojects {
                    h1 { "My Projects" }
                    hr;
                    div .list {
                        @for project in data::projects(input_path) {
                            div .project {
                                h1 .header { (project.title) }
                                @if let Some(figure) = project.figure {
                                    @match figure {
                                        data::ProjectFigure::Icon(icon) => {
                                            img src=(icon.output_path()) {}
                                        },
                                        data::ProjectFigure::Picture(path) => {
                                            img src=(path) {}
                                        }
                                    }
                                }

                                div .description {
                                    @for paragraph in project.description {
                                        p { (PreEscaped(paragraph)) }
                                    }
                                }

                                div .tags {
                                    @for language in project.tags.languages {
                                        div .tag .language {
                                            span .k {}
                                            span .v { (language) }
                                        }
                                    }
                                    @for tech in project.tags.tech {
                                        div .tag .tech {
                                            span .k {}
                                            span .v { (tech) }
                                        }
                                    }
                                }

                                div .links {
                                    div {
                                        img src=(icon!("Github", input_path).output_path()) {}
                                        span { "View on " a href=(project.links.github) {"GitHub"}}
                                    }
                                    @if let Some(homepage) = project.links.homepage {
                                        div {
                                            img src=(icon!("Info", input_path).output_path()) {}
                                            span { "See " a href=(homepage) {"Project Page"}}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div #contributions {
                    h1 { "Contributions" }
                    hr;
                    div .list {
                        @for project in data::contribution_projects(input_path) {
                            div .project {
                                h1 .header { (project.title) }
                                @if let Some(figure) = project.figure {
                                    @match figure {
                                        data::ProjectFigure::Icon(icon) => {
                                            img src=(icon.output_path()) {}
                                        },
                                        data::ProjectFigure::Picture(path) => {
                                            img src=(path) {}
                                        }
                                    }
                                }

                                div .contributions {
                                    @if project.contributions.len() == 1 {
                                        p { (PreEscaped(project.contributions[0])) }
                                    } @else {
                                        ul {
                                            @for contrib in project.contributions {
                                                li { (PreEscaped(contrib)) }
                                            }
                                        }
                                    }
                                }

                                div .tags {
                                    @for language in project.tags.languages {
                                        div .tag .language {
                                            span .k {}
                                            span .v { (language) }
                                        }
                                    }
                                    @for tech in project.tags.tech {
                                        div .tag .tech {
                                            span .k {}
                                            span .v { (tech) }
                                        }
                                    }
                                }

                                div .links {
                                    div {
                                        img src=(icon!("Github", input_path).output_path()) {}
                                        span { "View on " a href=(project.links.github) {"GitHub"}}
                                    }
                                    @if let Some(homepage) = project.links.homepage {
                                        div {
                                            img src=(icon!("Info", input_path).output_path()) {}
                                            span { "See " a href=(homepage) {"Project Page"}}
                                        }
                                    }
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

pub struct AboutPage;

impl Render for AboutPage {
    fn render(output_base_path: &Path, input_path: &Path) -> Vec<Page> {
        let (dir, name) = ("about", "index.html");

        let directory = output_base_path.join(dir);
        std::fs::create_dir_all(&directory).unwrap();

        let path = directory.join(name);

        let page = html!(
            main #aboutme {
                h1 { "About Me" }
                hr;
                div .content {
                    p {
                        "I'm Hannes Körber, a technology enthusiast currently living in Ansbach, Germany."
                    }

                    h1 { "Why I do what I am doing" }

                    p { "I started working with computers when I was around ten years old. In the beginning, I mainly used them for gaming, but got more and more interested in the internals --- how a computer actually works."}

                    p { "In school, I started programming (Visual Basic and C#) and was completely blown away that I could TELL the computer what to do, whatever it was. I then began building my own computers, and after the German \"Abitur\" (comparable to a high school degree), I started studying Information and Communications Technology at Friedrich-Alexander-Universität Erlangen-Nürnberg."}

                    p { "During my university years, I first came in contact with Linux. It was like discovering computers all over again. With Linux, I was free to do everything I wanted with my computer. A few months after having my first contact with Linux, I abandoned Windows for good and have not looked back. I quickly learned everything I could about Linux and computer science in general. By choosing computer science courses over Electrical engineering courses (which I still like and do as a hobby) I decided on my career path: Information Technology."}

                    h1 { "What I do in my free time" }

                    p {"I once read somewhere that you should have one hobby in each of the following three categories:"}

                    ul {
                        li {"A physical hobby, where you do some physical activity, preferably outside" }
                        li {"A creative hobby, where you create something new"}
                        li {"A hobby that makes you money"}
                    }

                    p {"Well, the last one for me is the one that comes most naturally: I take care of my own private "cloud" that encompasses a few services that for me replace google, dropbox etc. I use this to keep up to date on a lot of technologies that I cannot work with in my day-to-day job. So it indirectly makes me money by increasing my market value."}


                    h2 { "Sports"}

                    p { "For a physical hobby, I do not have a single one or even one I focus on, but I do numerous different things. Quantity over quality if you want:"}


                    ul {
                        li { span .buzzword {"Cycling"} ", from multi-day Bike touring to Mountain biking and some (very) mild Downhill. I bought a new bike mid of 2020 (a Radon Skeen Trail AL 2020, see picture), a mountain bike that has to fill all those roles in one"}

                        li { span .buzzword {"Hiking"} " and " span .buzzword {"Mountaineering"}}

                        li { span .buzzword {"Kayaking"} }

                        li {"Climbing, both " span .buzzword {"Boudering"} " and " span .buzzword {"Rock climbing"}
                        }

                        li { span .buzzword {"Weightlifting"} }

                        li { span .buzzword {"Running"} }
                    }

                    p {
                        "I started my climbing journey in 2022. I had been bouldering "
                        "a bit the years before, but in 2022 I started bouldering more "
                        "seriously. At some point, my brother took me to an indoor sports "
                        "climbing gym. Shortly afterwards, I took belaying and lead climbing "
                        "classes with the DAV (\"Deutscher Alpenverein\", German Alpine Club). "
                        "From there, it kind of snowballed and now I am bouldering and sport "
                        "climbing, both indoor and outdoors. I even dabbed into alpine climbing and "
                        "ice climbing."
                    }

                    h2 {"Creativity"}

                    p {(PreEscaped("The last kind of hobby&mdash;the creative one&mdash;is the one I have to force myself to do the most."))}

                    div .with-picture .picture-right {
                        figure {
                            img width="400px" src="/assets/images/guitar.jpg" {}
                            figcaption {"Amsterdam, July 2019"}
                        }
                        p {
                            "I have been learning playing " span .buzzword {"Guitar"} " since mid of 2019. I'm using "
                            a href="https://www.justinguitar.com/" {"JustinGuitar's course"}
                            " to get to know the basics. It's enough for some simple strumming, but don't expect any concerts right now. My goal is do be campfire-ready."
                        }
                    }

                    div .with-picture .picture-right {
                        figure {
                            img width="360px" src="/assets/images/yamaha-p45.jpg" {}
                            figcaption {"Yamaha P-45"}
                        }
                        div .content {
                            p {
                                "When I was younger, I also took some piano lessions with my grandma. After a ten-year hiatus, I've been relearning the " span .buzzword {"Piano"} " since beginning of 2020, after buying an electrical piano."
                            }

                            p {
                                "I bought a Yamaha P-45 (see picture). It has weighted keys and feels nearly like a \"real\" piano."
                            }
                        }
                    }

                    p {
                        "I also started attending a local church choir to work on my " span .buzzword {"Singing"} ", but with the COVID-19 situation this is currently not possible. Maybe that's better for everyone, because no one has to listen to me sing. ;)"
                    }

                    div .with-picture .picture-right {
                        figure {
                            img width="200px" src="/assets/images/chess.jpg" {}
                        }

                        div .content {
                            p {
                                "During Corona, I got back into " span .buzzword {"Chess"} ". My lichess rating is currently around 1450. My goal is ~1550, which is around the 50th percentile / median, meaning I'd have a 50/50 chance of beating a random opponent on lichess. Unfortunately, I'm mainly focussing on puzzles and not really playing longer games, mainly due to time contraints."
                            }

                            p {
                                "Chess is both fascinating (due to the rules' simplicity) and frustrating (due to having no skill ceiling). It can be a grind to improve, but then one day you have this one game that makes it worth it."
                            }
                        }
                    }

                    p {
                        "This webpage itself could also be considered a creative hobby. "
                        span .buzzword {"Writing"} " down technical stuff makes me internalize complex topics very efficiently."
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
