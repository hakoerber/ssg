use std::fs;
use std::io;
use std::path::Path;

pub fn copy_dir_all(
    src: impl AsRef<Path>,
    base: impl AsRef<Path>,
    dst: impl AsRef<Path>,
) -> io::Result<Vec<String>> {
    fs::create_dir_all(base.as_ref().join(&dst))?;

    let mut paths: Vec<String> = vec![];
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            paths.append(&mut copy_dir_all(
                entry.path(),
                base.as_ref(),
                dst.as_ref().join(entry.file_name()),
            )?);
        } else {
            fs::copy(
                entry.path(),
                base.as_ref().join(dst.as_ref().join(entry.file_name())),
            )?;
            paths.push(
                dst.as_ref()
                    .join(entry.file_name())
                    .to_str()
                    .unwrap()
                    .to_owned(),
            );
        }
    }
    Ok(paths)
}
