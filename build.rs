// build.rs

use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/styles.css");

    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();

    let dest_path = Path::new(&manifest_dir).join("static").join("index.css");

    eprintln!("Building tailwindcss to: {:?}", dest_path);

    let run = std::process::Command::new("bunx")
        .args(&[
            "tailwindcss",
            "-i",
            "src/styles.css",
            "-o",
            dest_path.to_str().unwrap(),
        ])
        .current_dir(manifest_dir)
        .spawn()
        .expect("failed to execute tailwind compiler")
        .wait()
        .expect("failed to wait for tailwind compiler");

    if run.success() {
        return;
    } else {
        panic!("failed to build tailwindcss");
    }
}

/*
    let mut tailwind_watcher = tokio::process::Command::new("bunx")
        .args(&[
            "tailwindcss",
            "-i",
            "src/routers/app/index.css",
            "-o",
            "static/index.css",
            "--watch",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = FramedRead::new(tailwind_watcher.stdout.take().unwrap(), LinesCodec::new())
        .map(|line| line.unwrap_or_default());

    let stderr = FramedRead::new(tailwind_watcher.stderr.take().unwrap(), LinesCodec::new())
        .map(|line| line.unwrap_or_default());

    let mut stream = stdout.merge(stderr);

    while let Some(msg) = stream.next().await {
        log::info!("tailwind: {}", msg);
    }
}*/
