use serde::Deserialize;
use std::env;
use std::path::{Path, PathBuf};
use std::{fs, process};

#[derive(Debug, Deserialize)]
enum FileType {
    Html,
    Svg,
    Css,
    Jpg,
    Png,
}

impl FileType {
    fn content_type(&self) -> &'static str {
        match self {
            Self::Html => "text/html;charset=utf-8",
            Self::Svg => "image/svg+xml",
            Self::Css => "text/css;charset=utf-8",
            Self::Jpg => "image/jpeg",
            Self::Png => "image/png",
        }
    }
}

#[derive(Debug, Deserialize)]
struct Page {
    path: String,
    filetype: FileType,
}

impl Page {
    fn content_type(&self) -> &'static str {
        self.filetype.content_type()
    }
}

#[derive(Debug, Deserialize)]
struct Manifest {
    content_directory: PathBuf,
    pages: Vec<Page>,
}

fn write_router(build_directory: &Path, manifest: &Manifest) {
    let mut code = String::new();

    code.push_str(
        r#"
        pub fn router() -> axum::Router {
            let router = axum::Router::new();
        "#,
    );

    for page in &manifest.pages {
        code.push_str(&format!(
            r#"let router = super::add_route!(router, "{path}", "{content_type}", "{file}");"#,
            path = Path::new("/").join(&page.path).to_str().unwrap(),
            content_type = page.content_type(),
            file = build_directory
                .join(&manifest.content_directory)
                .join(&page.path)
                .to_str()
                .unwrap()
        ));
        code.push('\n');
    }

    code.push_str(
        r#"
            router
        }
        "#,
    );

    fs::write(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("router.rs"),
        code,
    )
    .unwrap();
}

fn main() {
    let build_directory = PathBuf::from(env::var("GENERATOR_BUILD_DIRECTORY").unwrap());

    use std::{thread, time};
    thread::sleep(time::Duration::from_secs(10));

    let cmd = process::Command::new(env!("CARGO"))
        .args([
            "run",
            "--manifest-path",
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../generator/Cargo.toml")
                .to_str()
                .unwrap(),
            "--profile",
            if cfg!(debug_assertions) {
                "dev"
            } else {
                "release"
            },
            "--",
            "relaxed",
            build_directory.to_str().unwrap(),
        ])
        .output()
        .unwrap();

    if !cmd.status.success() {
        panic!(
            "{}",
            String::from_utf8(
                cmd.stdout
                    .into_iter()
                    .chain(cmd.stderr.into_iter())
                    .collect()
            )
            .unwrap()
        );
    }

    let manifest_path = build_directory.join("manifest.json");

    let manifest: Manifest =
        serde_json::from_str(&fs::read_to_string(&manifest_path).unwrap()).unwrap();

    write_router(&build_directory.as_ref(), &manifest);

    // this will always rebuild, as the manifest will be rewritten by the generator call above, so it will be changed
    // on every run. this could be optimized by checking the *content* of manifest (and writing checksums in there),
    // but rebuilding is fine
    println!("cargo:rerun-if-changed={}", manifest_path.to_str().unwrap());
}
