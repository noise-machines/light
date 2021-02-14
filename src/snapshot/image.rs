use crate::snapshot::manifest;
use crate::snapshot::Snapshot;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn clean_up(app: &nannou::prelude::App) {
    app.main_window().await_capture_frame_jobs().unwrap();

    fs::read_dir(images_folder_path())
        .unwrap()
        // Get the path for each entry.
        .map(|entry| entry.unwrap().path().to_str().unwrap().to_owned())
        // Only keep paths pointing to uncompressed images.
        .filter(|path| path.contains("uncompressed.tif"))
        .for_each(clean_up_uncompressed_file);
}

fn clean_up_uncompressed_file(uncompressed_path: String) {
    let compressed_path = uncompressed_path.replace(" uncompressed.tif", ".tif");

    // convert -compress lzw "image uncompressed.tif" image.tif
    let output = Command::new("convert")
        .arg("-compress")
        .arg("lzw")
        .arg(&uncompressed_path)
        .arg(compressed_path)
        .output()
        .unwrap();

    if !output.status.success() {
        let error_message = String::from_utf8(output.stderr).unwrap();
        panic!(error_message);
    }

    fs::remove_file(uncompressed_path).unwrap();
}

pub fn capture_frame(snapshot: &Snapshot, app: &nannou::prelude::App) {
    let image_path = uncompressed_path(&snapshot);
    app.main_window().capture_frame(image_path);
}

pub fn symlink_into_snapshots_directory(snapshot: &Snapshot) {
    let original_image_path = compressed_path(snapshot);
    let symlink_path = symlink_path(snapshot);
    std::os::unix::fs::symlink(original_image_path, symlink_path).unwrap();
}

fn symlink_path(snapshot: &Snapshot) -> PathBuf {
    manifest::folder()
        .join("snapshots")
        .join(&snapshot.name)
        .join(compressed_name(snapshot))
}

fn compressed_path(snapshot: &Snapshot) -> PathBuf {
    path(&compressed_name(snapshot))
}

fn uncompressed_path(snapshot: &Snapshot) -> PathBuf {
    path(&uncompressed_name(snapshot))
}

fn path(name: &str) -> PathBuf {
    images_folder_path().join(name)
}

fn images_folder_path() -> PathBuf {
    manifest::folder().join("images")
}

fn compressed_name(snapshot: &Snapshot) -> String {
    format!("{} {}{}", manifest::crate_name(), snapshot.name, ".tif")
}

fn uncompressed_name(snapshot: &Snapshot) -> String {
    format!(
        "{} {} uncompressed{}",
        manifest::crate_name(),
        snapshot.name,
        ".tif"
    )
}
