use std::fmt;
use std::path::Path;

use crate::icon;
use crate::icon::Icon;

pub const FULLNAME: &str = "Hannes Körber";

pub struct Certification {
    pub link: &'static str,
    pub title: &'static str,
    pub image: String,
}

pub fn certifications() -> Vec<Certification> {
    vec![Certification {
        link: "https://www.credly.com/badges/870a6345-ed4e-416e-9c46-c9af9c6d2c77/public_url",
        title: "AWS Certified Solutions Architect – Associate",
        image: "/assets/badges/aws-certified-solutions-architect-associate.png".into(),
    }]
}

pub struct Social {
    pub name: &'static str,
    pub link: String,
    pub icon: Box<dyn icon::Icon>,
    pub description: Option<&'static str>,
}

pub fn socials(input_path: &Path) -> Vec<Social> {
    vec![
        Social {
            name: "Github",
            link: "https://github.com/hakoerber".into(),
            icon: icon!("Github", input_path),
            description: None,
        },
        Social {
            name: "Linkedin",
            link: "https://www.linkedin.com/in/hannes-koerber".into(),
            icon: icon!("Linkedin", input_path),
            description: None,
        },
        Social {
            name: "Keybase",
            link: "https://keybase.io/hakoerber".into(),
            icon: icon!("Keybase", input_path),
            description: None,
        },
        Social {
            name: "E-Mail",
            link: "mailto:hannes.koerber@gmail.com".into(),
            icon: icon!("Email", input_path),
            description: Some("Send me an e-mail"),
        },
        Social {
            name: "RSS",
            link: "/rss.xml".into(),
            icon: icon!("Rss", input_path),
            description: Some("Follow my blog on RSS"),
        },
    ]
}

pub enum TechLevel {
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

pub struct Technology {
    pub name: &'static str,
    pub level: TechLevel,
    pub icon: Box<dyn icon::Icon>,
}

pub struct TechCategory {
    pub name: &'static str,
    pub technologies: Vec<Technology>,
}

pub fn tech_categories(input_path: &Path) -> Vec<TechCategory> {
    vec![
        TechCategory {
            name: "Containerization",
            technologies: vec![
                Technology {
                    name: "Kubernetes",
                    level: TechLevel::Pro,
                    icon: icon!("Kubernetes", input_path),
                },
                Technology {
                    name: "Docker",
                    level: TechLevel::Pro,
                    icon: icon!("Docker", input_path),
                },
                Technology {
                    name: "cri-o",
                    level: TechLevel::Normal,
                    icon: icon!("CriO", input_path),
                },
                Technology {
                    name: "Containerd",
                    level: TechLevel::Normal,
                    icon: icon!("Containerd", input_path),
                },
                Technology {
                    name: "OCI",
                    level: TechLevel::Normal,
                    icon: icon!("Oci", input_path),
                },
                Technology {
                    name: "Rancher",
                    level: TechLevel::Normal,
                    icon: icon!("Rancher", input_path),
                },
            ],
        },
        TechCategory {
            name: "Databases",
            technologies: vec![
                Technology {
                    name: "PostgreSQL",
                    level: TechLevel::Pro,
                    icon: icon!("Postgresql", input_path),
                },
                Technology {
                    name: "ElasticSearch",
                    level: TechLevel::Pro,
                    icon: icon!("Elasticsearch", input_path),
                },
                Technology {
                    name: "MySQL",
                    level: TechLevel::Pro,
                    icon: icon!("Mysql", input_path),
                },
                Technology {
                    name: "Redis",
                    level: TechLevel::Normal,
                    icon: icon!("Redis", input_path),
                },
                Technology {
                    name: "InfluxDB",
                    level: TechLevel::Normal,
                    icon: icon!("Influx", input_path),
                },
                Technology {
                    name: "SQLite",
                    level: TechLevel::Normal,
                    icon: icon!("Sqlite", input_path),
                },
            ],
        },
        TechCategory {
            name: "Configuration Management",
            technologies: vec![
                Technology {
                    name: "Terraform",
                    level: TechLevel::Pro,
                    icon: icon!("Terraform", input_path),
                },
                Technology {
                    name: "Ansible",
                    level: TechLevel::Pro,
                    icon: icon!("Ansible", input_path),
                },
                Technology {
                    name: "Pulumi",
                    level: TechLevel::Pro,
                    icon: icon!("Pulumi", input_path),
                },
                Technology {
                    name: "Packer",
                    level: TechLevel::Normal,
                    icon: icon!("Packer", input_path),
                },
                Technology {
                    name: "Puppet",
                    level: TechLevel::Normal,
                    icon: icon!("Puppet", input_path),
                },
                Technology {
                    name: "SaltStack",
                    level: TechLevel::Normal,
                    icon: icon!("Saltstack", input_path),
                },
            ],
        },
        TechCategory {
            name: "Web Development",
            technologies: vec![
                Technology {
                    name: "HTML",
                    level: TechLevel::Pro,
                    icon: icon!("Html5", input_path),
                },
                Technology {
                    name: "CCS",
                    level: TechLevel::Pro,
                    icon: icon!("Css", input_path),
                },
                Technology {
                    name: "JavaScript",
                    level: TechLevel::Normal,
                    icon: icon!("Javascript", input_path),
                },
                Technology {
                    name: "Flask",
                    level: TechLevel::Normal,
                    icon: icon!("Flask", input_path),
                },
                Technology {
                    name: "Svelte",
                    level: TechLevel::Normal,
                    icon: icon!("Svelte", input_path),
                },
                Technology {
                    name: "ReactJS",
                    level: TechLevel::Normal,
                    icon: icon!("Reactjs", input_path),
                },
            ],
        },
        TechCategory {
            name: "Programming",
            technologies: vec![
                Technology {
                    name: "Python",
                    level: TechLevel::Pro,
                    icon: icon!("Python", input_path),
                },
                Technology {
                    name: "Rust",
                    level: TechLevel::Pro,
                    icon: icon!("Rust", input_path),
                },
                Technology {
                    name: "Go",
                    level: TechLevel::Pro,
                    icon: icon!("Go", input_path),
                },
                Technology {
                    name: "TypeScript",
                    level: TechLevel::Normal,
                    icon: icon!("Typescript", input_path),
                },
                Technology {
                    name: "Bash",
                    level: TechLevel::Normal,
                    icon: icon!("Bash", input_path),
                },
                Technology {
                    name: "C",
                    level: TechLevel::Normal,
                    icon: icon!("C", input_path),
                },
            ],
        },
        TechCategory {
            name: "Observability",
            technologies: vec![
                Technology {
                    name: "Prometheus",
                    level: TechLevel::Pro,
                    icon: icon!("Prometheus", input_path),
                },
                Technology {
                    name: "Grafana",
                    level: TechLevel::Pro,
                    icon: icon!("Grafana", input_path),
                },
                Technology {
                    name: "Kibana",
                    level: TechLevel::Normal,
                    icon: icon!("Kibana", input_path),
                },
                Technology {
                    name: "OpsGenie",
                    level: TechLevel::Normal,
                    icon: icon!("Opsgenie", input_path),
                },
                Technology {
                    name: "OpenTelemetry",
                    level: TechLevel::Normal,
                    icon: icon!("Opentelemetry", input_path),
                },
                Technology {
                    name: "Jaeger",
                    level: TechLevel::Normal,
                    icon: icon!("Jaeger", input_path),
                },
            ],
        },
        TechCategory {
            name: "Development",
            technologies: vec![
                Technology {
                    name: "Git",
                    level: TechLevel::Pro,
                    icon: icon!("Git", input_path),
                },
                Technology {
                    name: "Neovim",
                    level: TechLevel::Pro,
                    icon: icon!("Neovim", input_path),
                },
                Technology {
                    name: "GitLab",
                    level: TechLevel::Normal,
                    icon: icon!("Gitlab", input_path),
                },
                Technology {
                    name: "GitHub",
                    level: TechLevel::Normal,
                    icon: icon!("Github", input_path),
                },
                Technology {
                    name: "OpenAPI",
                    level: TechLevel::Normal,
                    icon: icon!("Swagger", input_path),
                },
                Technology {
                    name: "Jira",
                    level: TechLevel::Normal,
                    icon: icon!("Jira", input_path),
                },
            ],
        },
        TechCategory {
            name: "Automation",
            technologies: vec![
                Technology {
                    name: "Drone",
                    level: TechLevel::Pro,
                    icon: icon!("Drone", input_path),
                },
                Technology {
                    name: "GitLab CI",
                    level: TechLevel::Pro,
                    icon: icon!("Gitlab", input_path),
                },
                Technology {
                    name: "Jenkins",
                    level: TechLevel::Normal,
                    icon: icon!("Jenkins", input_path),
                },
            ],
        },
        TechCategory {
            name: "Cloud Providers",
            technologies: vec![
                Technology {
                    name: "AWS",
                    level: TechLevel::Pro,
                    icon: icon!("Aws", input_path),
                },
                Technology {
                    name: "DigitalOcean",
                    level: TechLevel::Normal,
                    icon: icon!("Digitalocean", input_path),
                },
                Technology {
                    name: "Hetzner",
                    level: TechLevel::Normal,
                    icon: icon!("Hetzner", input_path),
                },
            ],
        },
        TechCategory {
            name: "Web Servers",
            technologies: vec![
                Technology {
                    name: "Nginx",
                    level: TechLevel::Pro,
                    icon: icon!("Nginx", input_path),
                },
                Technology {
                    name: "Apache",
                    level: TechLevel::Normal,
                    icon: icon!("Apache", input_path),
                },
                Technology {
                    name: "HAProxy",
                    level: TechLevel::Normal,
                    icon: icon!("Haproxy", input_path),
                },
                Technology {
                    name: "OpenResty",
                    level: TechLevel::Normal,
                    icon: icon!("Openresty", input_path),
                },
            ],
        },
        TechCategory {
            name: "Operating Systems",
            technologies: vec![
                Technology {
                    name: "CentOS",
                    level: TechLevel::Pro,
                    icon: icon!("Centos", input_path),
                },
                Technology {
                    name: "Debian",
                    level: TechLevel::Pro,
                    icon: icon!("Debian", input_path),
                },
                Technology {
                    name: "Arch Linux",
                    level: TechLevel::Pro,
                    icon: icon!("ArchLinux", input_path),
                },
                Technology {
                    name: "Ubuntu",
                    level: TechLevel::Normal,
                    icon: icon!("Ubuntu", input_path),
                },
                Technology {
                    name: "Fedora",
                    level: TechLevel::Normal,
                    icon: icon!("Fedora", input_path),
                },
                Technology {
                    name: "FreeBSD",
                    level: TechLevel::Normal,
                    icon: icon!("Freebsd", input_path),
                },
            ],
        },
        TechCategory {
            name: "Security",
            technologies: vec![
                Technology {
                    name: "Keycloak",
                    level: TechLevel::Pro,
                    icon: icon!("Keycloak", input_path),
                },
                Technology {
                    name: "OpenID Connect",
                    level: TechLevel::Pro,
                    icon: icon!("OpenidConnect", input_path),
                },
                Technology {
                    name: "GnuPG",
                    level: TechLevel::Normal,
                    icon: icon!("Gnupg", input_path),
                },
                Technology {
                    name: "Let's Encrypt",
                    level: TechLevel::Normal,
                    icon: icon!("Letsencrypt", input_path),
                },
                Technology {
                    name: "Wireshark",
                    level: TechLevel::Normal,
                    icon: icon!("Wireshark", input_path),
                },
                Technology {
                    name: "OpenVPN",
                    level: TechLevel::Normal,
                    icon: icon!("Openvpn", input_path),
                },
            ],
        },
        TechCategory {
            name: "Virtualization",
            technologies: vec![
                Technology {
                    name: "Libvirt / KVM",
                    level: TechLevel::Pro,
                    icon: icon!("Libvirt", input_path),
                },
                Technology {
                    name: "Vagrant",
                    level: TechLevel::Pro,
                    icon: icon!("Vagrant", input_path),
                },
                Technology {
                    name: "Qemu",
                    level: TechLevel::Normal,
                    icon: icon!("Qemu", input_path),
                },
            ],
        },
        TechCategory {
            name: "Storage",
            technologies: vec![
                Technology {
                    name: "Ceph",
                    level: TechLevel::Pro,
                    icon: icon!("Ceph", input_path),
                },
                Technology {
                    name: "ZFS",
                    level: TechLevel::Normal,
                    icon: icon!("Openzfs", input_path),
                },
            ],
        },
    ]
}

pub enum ProjectFigure {
    Icon(Box<dyn Icon>),
    Picture(&'static str),
}

pub struct ProjectTags {
    pub languages: Vec<&'static str>,
    pub tech: Vec<&'static str>,
}

pub struct ProjectLinks {
    pub github: &'static str,
    pub homepage: Option<&'static str>,
}

pub struct Project {
    pub title: &'static str,
    pub figure: Option<ProjectFigure>,
    pub description: Vec<&'static str>,
    pub tags: ProjectTags,
    pub links: ProjectLinks,
}

pub struct ContributionProject {
    pub title: &'static str,
    pub figure: Option<ProjectFigure>,
    pub contributions: Vec<&'static str>,
    pub tags: ProjectTags,
    pub links: ProjectLinks,
}

pub fn projects(input_path: &Path) -> Vec<Project> {
    vec![
        Project {
            title: "git-repo-manager",
            figure: Some(ProjectFigure::Icon(icon!("Git", input_path))),
            description: vec!["A command-line tool to manage local git repositories"],
            tags: ProjectTags {
                languages: vec!["Rust"],
                tech: vec!["Libgit2", "Toml"],
            },
            links: ProjectLinks {
                github: "https://github.com/hakoerber/git-repo-manager",
                homepage: Some("https://hakoerber.github.io/git-repo-manager/"),
            },
        },
        Project {
            title: "prometheus-restic-backblaze",
            figure: Some(ProjectFigure::Icon(icon!("Backblaze", input_path))),
            description: vec!["A prometheus exporter that reports restic backup ages for Backblaze"],
            tags: ProjectTags {
                languages: vec!["Python"],
                tech: vec!["Prometheus", "Restic"],
            },
            links: ProjectLinks {
                github: "https://github.com/hakoerber/prometheus-restic-backblaze",
                homepage: Some("https://github.com/hakoerber/prometheus-restic-backblaze"),
            },
        },
        Project {
            title: "virt-bootstrap",
            figure: None,
            description: vec!["A script that bootstraps a new libvirt VM using cobbler"],
            tags: ProjectTags {
                languages: vec!["Python"],
                tech: vec!["Libvirt", "Cobbler"],
            },
            links: ProjectLinks {
                github: "https://github.com/hakoerber/virt-bootstrap",
                homepage: None,
            },
        },
        Project {
            title: "aws-glacier-backup",
            figure: Some(ProjectFigure::Icon(icon!("AwsS3", input_path))),
            description: vec!["A bash script that uploads gzip’ed, gpg encrypted backups to AWS glacier"],
            tags: ProjectTags {
                languages: vec!["Bash"],
                tech: vec!["AWS S3", "GPG"],
            },
            links: ProjectLinks {
                github: "https://github.com/hakoerber/aws-glacier-backup",
                homepage: None,
            },
        },
        Project {
            title: "guitar-practice",
            figure: Some(ProjectFigure::Picture("/assets/images/guitar-closeup.jpg")),
            description: vec![concat!(
                "A simple python script that gives me a series of guitar chords ",
                "to practice chord transitions, with customizable rate of change"
            )],
            tags: ProjectTags {
                languages: vec!["Python"],
                tech: vec![],
            },
            links: ProjectLinks {
                github: "https://github.com/hakoerber/guitar-practice",
                homepage: None,
            },
        },
        Project {
            title: "checkconn",
            figure: None,
            description:
                vec!["Utiliy that continuously monitors the internet connection and reports downtimes"],
            tags: ProjectTags {
                languages: vec!["Bash"],
                tech: vec![],
            },
            links: ProjectLinks {
                github: "https://github.com/hakoerber/checkconn",
                homepage: None,
            },
        },
        Project {
            title: "packager",
            figure: None,
            description: vec![concat!(
                "A learning project that can be used to manage packing lists for ",
                "trips, considering duration, weather and other factors."
            ), "I mainly wrote this to play around with Flask and Elm"],
            tags: ProjectTags {
                languages: vec!["Rust", "Python", "Elm", "Javascript", "Svelte"],
                tech: vec!["HTMX", "Flask", "SQlite"],
            },
            links: ProjectLinks {
                github: "https://github.com/hakoerber/packager",
                homepage: None,
            },
        },
        Project {
            title: "salt-nginx-letsencrypt",
            figure: Some(ProjectFigure::Icon(icon!("Letsencrypt", input_path))),
            description: vec!["A SaltStack nginx formula that also enables automated letsencrypt certificate management"],
            tags: ProjectTags {
                languages: vec!["Python"],
                tech: vec!["SaltStack", "LetsEncrypt", "Nginx"],
            },
            links: ProjectLinks {
                github: "https://github.com/hakoerber/salt-nginx-letsencrypt",
                homepage: None,
            },
        },
        Project {
            title: "ansible-roles",
            figure: Some(ProjectFigure::Icon(icon!("Ansible", input_path))),
            description: vec!["A collection of ansible roles, e.g. for libvirt, networking, OpenVPN"],
            tags: ProjectTags {
                languages: vec!["YAML"],
                tech: vec!["Ansible"],
            },
            links: ProjectLinks {
                github: "https://github.com/hakoerber/ansible-roles",
                homepage: None,
            },
        },
        Project {
            title: "salt-states",
            figure: Some(ProjectFigure::Icon(icon!("Saltstack", input_path))),
            description: vec![
                concat!(
                    "A big collection of saltstack states that I used for my ",
                    "homelab."),
                concat!("It contains configuration for a bunch of different ",
                    "services, e.g. elasticsearch, dovecot, grafana, influxdb, jenkins, ",
                    "kibana, nginx, owncloud, postgresql, ssh and a lot of others."
                )],
            tags: ProjectTags {
                languages: vec!["YAML", "Jinja2"],
                tech: vec!["SaltStack"],
            },
            links: ProjectLinks {
                github: "https://github.com/hakoerber/salt-states",
                homepage: None,
            },
        },
        Project {
            title: "wifiqr",
            figure: Some(ProjectFigure::Picture("/assets/images/qrcode-example.png")),
            description: vec!["A script that generates QR codes for easy WiFi access"],
            tags: ProjectTags {
                languages: vec!["Bash"],
                tech: vec![],
            },
            links: ProjectLinks {
                github: "https://github.com/hakoerber/wifiqr",
                homepage: None,
            },
        },
        Project {
            title: "syncrepo",
            figure: None,
            description: vec![concat!(
                "A python script to create and maintain a local YUM/DNF package ",
                "repository for CentOS."),
            concat!("Can be used to keep a mirror up to date with ",
                "<code>cron(8)</code>.")],
            tags: ProjectTags {
                languages: vec!["Python"],
                tech: vec!["DNF"],
            },
            links: ProjectLinks {
                github: "https://github.com/hakoerber/syncrepo",
                homepage: None,
            },
        },
    ]
}

pub fn contribution_projects(input_path: &Path) -> Vec<ContributionProject> {
    vec![
        ContributionProject {
            title: "Prometheus Node Exporter",
            figure: Some(ProjectFigure::Icon(icon!("Prometheus", input_path))),
            contributions: vec![
                "Add label to NFS metrics containing the NFS protocol (<code>tcp/udp</code>)",
            ],

            tags: ProjectTags {
                languages: vec!["Go"],
                tech: vec!["Prometheus", "NFS"],
            },
            links: ProjectLinks {
                github: "https://github.com/prometheus/node_exporter",
                homepage: None,
            },
        },
        ContributionProject {
            title: "Kubespray",
            figure: Some(ProjectFigure::Icon(icon!("Kubernetes", input_path))),
            contributions: vec![
                "Fix issues with continuous regeneration of etcd TLS cerificates",
                "Fix incorrect directory mode for etcd TLS certificates",
            ],

            tags: ProjectTags {
                languages: vec!["YAML"],
                tech: vec!["Kubernetes", "Ansible"],
            },
            links: ProjectLinks {
                github: "https://github.com/kubernetes-sigs/kubespray/",
                homepage: None,
            },
        },
        ContributionProject {
            title: "SaltStack",
            figure: Some(ProjectFigure::Icon(icon!("Saltstack", input_path))),
            contributions: vec![
                "Expand the <code>firewalld</code> module for interfaces, sources, services and zones",
                "Fix the reactor engine not being loaded when not explicitly configured",
            ],
            tags: ProjectTags {
                languages: vec!["Python"],
                tech: vec!["SaltStack", "Firewalld"],
            },
            links: ProjectLinks {
                github: "https://github.com/saltstack/salt",
                homepage: None,
            },
        },        
        ContributionProject {
            title: "Vagrant",
            figure: Some(ProjectFigure::Icon(icon!("Vagrant", input_path))),
            contributions: vec![
                "Renew DHCP lease on hostname change for Debian guests",
                "Fix hostname entry in <code>/etc/hosts</code> for Debian guests",
            ],
            tags: ProjectTags {
                languages: vec!["Ruby"],
                tech: vec!["Vagrant"],
            },
            links: ProjectLinks {
                github: "https://github.com/hashicorp/vagrant",
                homepage: None,
            },
        },
        ContributionProject {
            title: "Prometheus procfs",
            figure: Some(ProjectFigure::Icon(icon!("Prometheus", input_path))),
            contributions: vec![
                "Add exporting of a new field containing the NFS protocol (required for the node exporter change)",
                "Fix parsing of the <code>xprt</code> lines in <code>mountstats</code> to enable metric exports for UDP mounts",
            ],
            tags: ProjectTags {
                languages: vec!["Go"],
                tech: vec!["Prometheus", "NFS"],
            },
            links: ProjectLinks {
                github: "https://github.com/prometheus/procfs",
                homepage: None,
            },
        },
        ContributionProject {
            title: "The Lost Son",
            figure: Some(ProjectFigure::Picture("/assets/images/lostson.jpg")),
            contributions: vec![
                "Our contribution to the Global Game Jam 2018!",
            ],
            tags: ProjectTags {
                languages: vec!["Javascript"],
                tech: vec!["Phaser"],
            },
            links: ProjectLinks {
                github: "https://github.com/niklas-heer/the-lost-son",
                homepage: None,
            },
        },
    ]
}
