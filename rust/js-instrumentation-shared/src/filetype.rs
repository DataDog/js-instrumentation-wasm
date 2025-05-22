pub fn filename_is_jsx(filename: &str) -> bool {
    filename.ends_with(".jsx")
        || filename.ends_with(".cjsx")
        || filename.ends_with(".mjsx")
        || filename.ends_with(".tsx")
        || filename.ends_with(".ctsx")
        || filename.ends_with(".mtsx")
}

pub fn filename_is_typescript(filename: &str) -> bool {
    filename.ends_with(".ts")
        || filename.ends_with(".cts")
        || filename.ends_with(".mts")
        || filename.ends_with(".tsx")
        || filename.ends_with(".ctsx")
        || filename.ends_with(".mtsx")
}

pub fn filename_is_explicitly_cjs(filename: &str) -> bool {
    filename.ends_with(".cjs")
        || filename.ends_with(".cjsx")
        || filename.ends_with(".cts")
        || filename.ends_with(".ctsx")
}

pub fn filename_is_explicitly_esm(filename: &str) -> bool {
    filename.ends_with(".mjs")
        || filename.ends_with(".mjsx")
        || filename.ends_with(".mts")
        || filename.ends_with(".mtsx")
}
