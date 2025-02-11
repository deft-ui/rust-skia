use crate::build_support::cargo;

/// Returns `true` if the download of prebuilt binaries should be forced.
///
/// This can be used to test and download prebuilt binaries from within a repository build.
/// If this environment variable is not set, binaries are downloaded from crate builds only.
pub fn force_skia_binaries_download() -> bool {
    cargo::env_var("FORCE_SKIA_BINARIES_DOWNLOAD").is_some()
}

/// The URL template to download the Skia binaries from.
///
/// `{tag}` will be replaced by the Tag (usually the released skia-binding's crate's version).
/// `{key}` will be replaced by the Key (a combination of the repository hash, target, and features).
///
/// `file://` URLs are supported for local testing.
pub fn skia_binaries_url() -> Option<String> {
    cargo::env_var("SKIA_BINARIES_URL")
}

/// The default URL template to download the binaries from.
pub fn skia_binaries_url_default() -> String {
    let mut dist_url = cargo::env_var("DEFT_DIST_URL")
        .unwrap_or("https://github.com/deft-ui/skia-binaries/releases/download".to_string());
    if dist_url.ends_with("/") {
        dist_url = dist_url[..dist_url.len() - 1].to_string();
    }
    format!("{}/{}", dist_url, "{tag}/skia-binaries-{key}.tar.gz")
}

/// Force to build Skia, even if there is a binary available.
pub fn force_skia_build() -> bool {
    cargo::env_var("FORCE_SKIA_BUILD").is_some()
}
