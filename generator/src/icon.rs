use super::fs;
use super::{FileType, Page};
use std::path::{Path, PathBuf};

pub trait Icon {
    fn filename(&self) -> &'static str;
    fn local_path(&self, input_dir: &Path) -> PathBuf;
    fn output_path(&self) -> String;
}

pub mod m {
    #[macro_export]
    macro_rules! icon {
        ($name:literal, $input_path:ident) => {{
            use paste::paste;
            use std::path::PathBuf;

            paste! {
                struct [<Icon $name>](());
            }

            impl $crate::icon::Icon for paste! { [<Icon $name>] } {
                fn filename(&self) -> &'static str {
                    concat!($name, ".svg")
                }

                fn local_path(&self, input_dir: &Path) -> PathBuf {
                    input_dir.join("static/icons/").join(self.filename())
                }

                fn output_path(&self) -> String {
                    Path::new("/icons")
                        .join(self.filename())
                        .to_str()
                        .unwrap()
                        .to_owned()
                }
            }

            fn verify(icon: &paste! { [<Icon $name>] }, input_dir: &Path) -> Result<(), String> {
                let path = icon.local_path(input_dir);
                if !path.exists() {
                    return Err(format!("icon {} at {path:?} does not exist", $name).into());
                }
                // SAFETY: not doing any multithreading
                unsafe { $crate::SEEN_ICONS.push(icon.filename()) };
                Ok(())
            }

            let icon = Box::new(paste! { [<Icon $name>](()) });
            verify(&icon, $input_path).unwrap();
            icon
        }};
    }
}

pub enum UnusedIconFiles {
    Allow,
    Deny,
}

pub struct IconsUnverified;

impl IconsUnverified {
    pub fn verify_all(allow_unused: UnusedIconFiles, input_path: &Path) -> IconsVerified {
        if let UnusedIconFiles::Deny = allow_unused {
            for local_file in std::fs::read_dir(input_path.join("static/icons"))
                .unwrap()
                .map(|entry| {
                    let entry = entry.unwrap();
                    if !entry.metadata().unwrap().is_file() {
                        panic!("not a file: {entry:?}")
                    }
                    entry.file_name().into_string().unwrap()
                })
            {
                // SAFETY: not doing any multithreading
                if !unsafe { crate::SEEN_ICONS.contains(&local_file.as_str()) } {
                    panic!("superfluous icon file {local_file:?}")
                }
            }
        }

        IconsVerified(())
    }
}

pub struct IconsVerified(());

impl IconsVerified {
    pub fn copy_all(self, output_base_path: &Path, input_path: &Path) -> Vec<Page> {
        fs::copy_dir_all(
            input_path.join("./static/icons"),
            output_base_path,
            Path::new("icons"),
        )
        .unwrap()
        .into_iter()
        .map(|path| Page {
            filetype: FileType::detect(&path).unwrap(),
            path,
        })
        .collect()
    }
}
