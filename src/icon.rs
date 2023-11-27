use super::fs;
use std::path::Path;

pub trait Icon {
    fn filename(&self) -> String;
    fn path(&self, output_base_path: &Path) -> String;
}

pub mod m {
    #[macro_export]
    macro_rules! icon {
        ($i:literal) => {{
            paste::paste! {
                struct [<Icon $i>](());
            }

            impl $crate::icon::Icon for paste::paste! { [<Icon $i>] } {
                fn filename(&self) -> String {
                    let mut s = String::from($i);
                    s.push_str(".svg");
                    s
                }

                fn path(&self, output_base_path: &Path) -> String {
                    let output_path = output_base_path.join("icons").join(self.filename());
                    let local_path = Path::new("static/icons").join(self.filename());
                    if !local_path.exists() {
                        panic!("icon at {local_path:?} does not exist")
                    }
                    output_path.to_str().unwrap().to_owned()
                }
            }
            let x: paste::paste! { [<Icon $i>] } = paste::paste! {
                [<Icon $i>](())
            };
            Box::new(x)
        }};
    }
}

pub enum UnusedIconFiles {
    Allow,
    Deny,
}

pub struct IconsUnverified;

impl IconsUnverified {
    pub fn verify_all(_allow_unused: UnusedIconFiles) -> IconsVerified {
        // for icon in IconInner::VARIANTS {
        //     let local_path = Path::new("static/icons").join(format!("{icon}.svg"));
        //     if !local_path.exists() {
        //         panic!("icon at {local_path:?} does not exist")
        //     }
        // }

        // if let UnusedIconFiles::Deny = allow_unused {
        //     let filenames: Vec<String> = IconInner::VARIANTS
        //         .iter()
        //         .map(|icon| format!("{icon}.svg"))
        //         .collect();

        //     for local_file in std::fs::read_dir("static/icons").unwrap().map(|entry| {
        //         let entry = entry.unwrap();
        //         if !entry.metadata().unwrap().is_file() {
        //             panic!("not a file: {entry:?}")
        //         }
        //         entry.file_name().into_string().unwrap()
        //     }) {
        //         if !filenames.contains(&local_file) {
        //             panic!("superfluous icon file {local_file:?}")
        //         }
        //     }
        // }

        IconsVerified(())
    }
}

pub struct IconsVerified(());

impl IconsVerified {
    pub fn copy_all(self, output_base_path: &Path) {
        fs::copy_dir_all("./static/icons", output_base_path.join("icons")).unwrap();
    }
}
