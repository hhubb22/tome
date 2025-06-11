// src/epub/rewriter.rs
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use path_clean::PathClean;
use pathdiff;
use path_slash::PathBufExt;
use urlencoding;

/// A private helper to create a normalized, lowercase key for path lookups.
pub(super) fn normalize_path_key(path_str: &str) -> String {
    urlencoding::decode(path_str)
        .map(|s| s.into_owned())
        .unwrap_or_else(|_| path_str.to_string())
        .to_lowercase()
}

/// Rewrites a relative link from its context in the EPUB to its new context in the static site.
pub(super) fn rewrite_link(
    original_link: &str,
    source_epub_dir: &Path,
    source_site_path: &Path,
    path_map: &HashMap<String, PathBuf>,
) -> Option<String> {
    if original_link.starts_with(|c: char| c == '#' || c == '/') || original_link.contains(':') {
        return None; // Absolute paths, fragments, or external URLs are not rewritten
    }
    
    // Split the link into file path and fragment (anchor) parts
    let (file_part, fragment_part) = if let Some(hash_pos) = original_link.find('#') {
        let (file_path, fragment) = original_link.split_at(hash_pos);
        (file_path, Some(fragment)) // fragment includes the '#'
    } else {
        (original_link, None)
    };
    
    let decoded_link = urlencoding::decode(file_part).unwrap_or_else(|_| file_part.into()).into_owned();
    let resolved_epub_path = source_epub_dir.join(decoded_link);
    
    // Normalize the final resolved path to lowercase before lookup.
    let key = resolved_epub_path.clean().to_slash_lossy().to_lowercase();

    if let Some(target_site_path) = path_map.get(&key) {
        let source_site_dir = source_site_path.parent().unwrap_or_else(|| Path::new(""));
        if let Some(new_relative_path) = pathdiff::diff_paths(target_site_path, source_site_dir) {
            let mut result = new_relative_path.to_slash().map(|s| s.into_owned())?;
            // Append the fragment part if it exists
            if let Some(fragment) = fragment_part {
                result.push_str(fragment);
            }
            return Some(result);
        }
    }

    // Reduce noise by ignoring common font file extensions that might be listed as fallbacks in CSS.
    if !matches!(Path::new(file_part).extension().and_then(|s| s.to_str()), Some("ttf" | "otf" | "woff" | "woff2")) {
        eprintln!("⚠️  Could not resolve link '{}' (normalized to '{}') from asset '{}'", original_link, key, source_epub_dir.to_string_lossy());
    }
    None
}