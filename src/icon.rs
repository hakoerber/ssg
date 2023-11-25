use super::fs;
use std::path::Path;

use strum::VariantNames;
use strum_macros::{EnumVariantNames, IntoStaticStr};

#[derive(IntoStaticStr, EnumVariantNames, Clone)]
#[cfg_attr(debug_assertions, allow(dead_code))]
#[cfg_attr(not(debug_assertions), deny(dead_code))]
pub enum Icon {
    Ansible,
    Apache,
    ArchLinux,
    AwsEc2,
    AwsEcs,
    AwsEfs,
    AwsIam,
    AwsLambda,
    AwsRds,
    AwsRoute53,
    AwsS3,
    AwsVpc,
    Aws,
    Backblaze,
    Bash,
    Bulma,
    C,
    Cadvisor,
    Centos,
    Ceph,
    Checkmk,
    CloudDownload,
    Container,
    Containerd,
    CriO,
    Csharp,
    Css,
    Debian,
    Digitalocean,
    Django,
    Docker,
    Dovecot,
    Drone,
    Elasticsearch,
    Elm,
    Email,
    Fedora,
    Flask,
    Foreman,
    Freebsd,
    Gears,
    Git,
    Github,
    Gitlab,
    Gnupg,
    Go,
    Grafana,
    Haproxy,
    Helm,
    Hetzner,
    Html5,
    Hugo,
    Influx,
    Jaeger,
    Java,
    Javascript,
    Jenkins,
    Jira,
    Keybase,
    Keycloak,
    Kibana,
    Kubernetes,
    Latex,
    Letsencrypt,
    Libvirt,
    Linkedin,
    Logstash,
    Lxc,
    MagnifyingGlass,
    Mongodb,
    Mysql,
    Neovim,
    Network,
    Nginx,
    Nmap,
    Oauth,
    Oci,
    OpenidConnect,
    Openresty,
    Openstack,
    Opentelemetry,
    Openvpn,
    Openzfs,
    Opsgenie,
    Ovirt,
    Packer,
    Pfsense,
    Php,
    Postfix,
    Postgresql,
    Prometheus,
    Pulumi,
    Puppet,
    Python,
    Qemu,
    Rancher,
    Reactjs,
    Redis,
    Rss,
    Ruby,
    Rundeck,
    Rust,
    Saltstack,
    Shield,
    Sqlite,
    Svelte,
    Swagger,
    Systemd,
    Terraform,
    Typescript,
    Ubuntu,
    Vagrant,
    VisualStudioCode,
    Wireshark,
}

pub enum UnusedIconFiles {
    Allow,
    Deny,
}

pub struct IconsUnverified;

impl IconsUnverified {
    pub fn verify_all(allow_unused: UnusedIconFiles) -> IconsVerified {
        for icon in Icon::VARIANTS {
            let local_path = Path::new("static/icons").join(format!("{icon}.svg"));
            if !local_path.exists() {
                panic!("icon at {local_path:?} does not exist")
            }
        }

        if let UnusedIconFiles::Deny = allow_unused {
            let filenames: Vec<String> = Icon::VARIANTS
                .iter()
                .map(|icon| format!("{icon}.svg"))
                .collect();

            for local_file in std::fs::read_dir("static/icons").unwrap().map(|entry| {
                let entry = entry.unwrap();
                if !entry.metadata().unwrap().is_file() {
                    panic!("not a file: {entry:?}")
                }
                entry.file_name().into_string().unwrap()
            }) {
                if !filenames.contains(&local_file) {
                    panic!("superfluous icon file {local_file:?}")
                }
            }
        }

        IconsVerified(())
    }
}

pub struct IconsVerified(());

impl IconsVerified {
    pub fn copy_all(self, output_base_path: &Path) {
        fs::copy_dir_all("./static/icons", output_base_path.join("icons")).unwrap();
    }
}

impl Icon {
    pub fn filename(&self) -> String {
        <Self as Into<&'static str>>::into(self.clone()).to_owned() + ".svg"
    }

    pub fn path(&self, output_base_path: &Path) -> String {
        let output_path = output_base_path.join("icons").join(self.filename());

        let local_path = Path::new("static/icons").join(self.filename());
        if !local_path.exists() {
            panic!("icon at {local_path:?} does not exist")
        }
        output_path.to_str().unwrap().to_owned()
    }
}
