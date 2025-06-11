use std::collections::HashMap;
use std::fs;
use std::io::{copy, Read, Write};
use std::path::{Path, PathBuf};

use lazy_static::lazy_static;
use lol_html::{element, rewrite_str, RewriteStrSettings};
use lol_html::html_content::Element;
use pathdiff;
use path_slash::PathBufExt;
use regex::{Captures, Regex};

use crate::epub::{model::ManifestItem, Epub};
use crate::epub::rewriter::{normalize_path_key, rewrite_link};
use crate::error::{AppError, Result};

const TEMPLATE_HTML: &str = include_str!("../../static/template.html");
const STYLES_CSS: &str = include_str!("../../static/styles.css");
const STYLES_CSS_FILENAME: &str = "styles.css";

lazy_static! {
    static ref CSS_URL_RE: Regex = Regex::new(r#"url\((.*?)\)"#).unwrap();
}

/// Orchestrates the conversion of an EPUB to a static website.
pub struct SiteGenerator<'a> {
    epub: &'a mut Epub,
    output_dir: &'a Path,
    no_nav: bool,
    path_map: HashMap<String, PathBuf>,
    spine_items: Vec<ManifestItem>,
}

impl<'a> SiteGenerator<'a> {
    pub fn new(epub: &'a mut Epub, output_dir: &'a Path, no_nav: bool) -> Self {
        Self { epub, output_dir, no_nav, path_map: HashMap::new(), spine_items: Vec::new() }
    }

    pub fn run(&mut self) -> Result<()> {
        self.prepare_directory()?;
        println!("   ├── Prepared output directory.");
        self.build_path_map_and_spine();
        println!("   ├── Built path map and identified spine content.");
        self.process_and_copy_assets()?;
        println!("   ├── Processed and copied all assets (images, fonts, CSS).");
        self.copy_static_assets()?;
        println!("   ├── Injected template assets.");
        self.transform_content_documents()?;
        println!("   ├── Transformed HTML content and applied templates.");
        self.generate_toc()?;
        println!("   ├── Generated table of contents (index.html).");
        Ok(())
    }

    fn prepare_directory(&self) -> Result<()> {
        if self.output_dir.exists() { fs::remove_dir_all(self.output_dir)?; }
        fs::create_dir_all(self.output_dir.join("assets"))?;
        fs::create_dir_all(self.output_dir.join("chapters"))?;
        Ok(())
    }

    fn build_path_map_and_spine(&mut self) {
        let manifest_map: HashMap<_, _> = self.epub.manifest().items.iter()
            .map(|item| (normalize_path_key(&item.id), item.clone()))
            .collect();

        self.spine_items = self.epub.spine().item_refs.iter()
            .filter_map(|item_ref| manifest_map.get(&normalize_path_key(&item_ref.idref)).cloned())
            .collect();

        for item in &self.epub.manifest().items {
            let key = normalize_path_key(&item.href);
            let new_path = match item.media_type.as_str() {
                "application/xhtml+xml" | "text/html" => {
                    let mut p = PathBuf::from("chapters");
                    let file_path = PathBuf::from(&key);
                    p.push(file_path.file_name().unwrap_or_else(|| "unknown.html".as_ref()));
                    p.set_extension("html");
                    p
                }
                _ => {
                    let mut p = PathBuf::from("assets");
                    let file_path = PathBuf::from(&key);
                    p.push(file_path.file_name().unwrap_or_else(|| "unknown".as_ref()));
                    p
                }
            };
            self.path_map.insert(key, new_path);
        }
    }

    fn process_and_copy_assets(&mut self) -> Result<()> {
        let manifest_items = self.epub.manifest().items.clone();
        for item in &manifest_items {
            match item.media_type.as_str() {
                "application/xhtml+xml" | "text/html" => {}
                "text/css" => self.transform_and_copy_css(item)?,
                _ => self.copy_single_asset(item)?,
            }
        }
        Ok(())
    }
    
    fn copy_single_asset(&mut self, item: &ManifestItem) -> Result<()> {
        let key = normalize_path_key(&item.href);
        if let Some(dest_rel_path) = self.path_map.get(&key) {
            let dest_path = self.output_dir.join(dest_rel_path);
            if let Some(parent) = dest_path.parent() { fs::create_dir_all(parent)?; }
            let mut archive_file = self.epub.read_by_manifest_item(item)?;
            let mut dest_file = fs::File::create(&dest_path)?;
            copy(&mut archive_file, &mut dest_file)?;
        }
        Ok(())
    }

    fn transform_and_copy_css(&mut self, item: &ManifestItem) -> Result<()> {
        let mut original_css = String::new();
        self.epub.read_by_manifest_item(item)?.read_to_string(&mut original_css)?;

        let key = normalize_path_key(&item.href);
        let source_epub_path = PathBuf::from(&key);
        let source_epub_dir = source_epub_path.parent().unwrap_or_else(|| Path::new(""));
        
        let source_site_path = self.path_map.get(&key).ok_or_else(|| AppError::InvalidEpubFormat(format!("Path not found in map for CSS href: {}", key)))?;

        let path_map = self.path_map.clone();
        let source_epub_dir = source_epub_dir.to_path_buf();
        let source_site_path = source_site_path.to_path_buf();

        let rewritten_css = CSS_URL_RE.replace_all(&original_css, |caps: &Captures| {
            let captured_content = &caps[1];
            let original_url = captured_content.trim_matches(|c| c == '\'' || c == '"');
            let new_url = rewrite_link(original_url, &source_epub_dir, &source_site_path, &path_map).unwrap_or_else(|| original_url.to_string());
            format!("url('{}')", new_url)
        });

        if let Some(dest_rel_path) = self.path_map.get(&key) {
            let dest_path = self.output_dir.join(dest_rel_path);
            let mut dest_file = fs::File::create(&dest_path)?;
            dest_file.write_all(rewritten_css.as_bytes())?;
        }
        Ok(())
    }

    fn copy_static_assets(&self) -> Result<()> {
        let css_path = self.output_dir.join("assets").join(STYLES_CSS_FILENAME);
        fs::write(css_path, STYLES_CSS)?;
        Ok(())
    }

    fn transform_content_documents(&mut self) -> Result<()> {
        for (i, item) in self.spine_items.clone().iter().enumerate() { self.transform_html_file(item, i)?; }
        Ok(())
    }

    fn generate_toc(&mut self) -> Result<()> {
        let book_title = self.epub.metadata().title.first().cloned().unwrap_or_else(|| "目录".to_string());
        let mut toc_html = format!("<h1 class=\"toc-title\">{}</h1>\n<ul class=\"toc\">", book_title);

        for (index, item) in self.spine_items.iter().enumerate() {
            let key = normalize_path_key(&item.href);
            if let Some(site_path) = self.path_map.get(&key) {
                let chapter_title = site_path.file_stem().unwrap_or_default().to_string_lossy();
                let display_title = if chapter_title.is_empty() || chapter_title == "unknown" {
                    format!("第 {} 章", index + 1)
                } else {
                    chapter_title.to_string()
                };
                toc_html.push_str(&format!("<li><a href=\"{}\">{}</a></li>\n", site_path.to_slash_lossy(), display_title));
            }
        }
        toc_html.push_str("</ul>");

        let styles_path = format!("assets/{}", STYLES_CSS_FILENAME);
        
        // 为TOC页面添加特殊样式处理
        let final_html = TEMPLATE_HTML
            .replace("{{ title }}", &book_title)
            .replace("{{ body }}", &toc_html)
            .replace("{{ nav }}", "")
            .replace("{{ styles_path }}", &styles_path)
            .replace("<main class=\"content\">", "<main class=\"content toc-page\">");

        fs::write(self.output_dir.join("index.html"), final_html)?;
        Ok(())
    }

    fn transform_html_file(&mut self, item: &ManifestItem, spine_index: usize) -> Result<()> {
        let mut original_html_bytes = Vec::new();
        self.epub.read_by_manifest_item(item)?.read_to_end(&mut original_html_bytes)?;
        
        let key = normalize_path_key(&item.href);
        let source_epub_path = PathBuf::from(&key);
        let source_site_path = self.path_map.get(&key).ok_or_else(|| AppError::InvalidEpubFormat(format!("Path not found in map for href: {}", key)))?.clone();
        let settings = self.get_html_rewrite_settings(&source_epub_path, &source_site_path);

        let rewritten_body_str = rewrite_str(&String::from_utf8_lossy(&original_html_bytes), settings)?;
        
        let nav_html = self.create_nav_html(spine_index, &source_site_path);
        let source_site_dir = source_site_path.parent().unwrap_or_else(|| Path::new(""));
        let styles_rel_path = pathdiff::diff_paths(&self.output_dir.join("assets").join(STYLES_CSS_FILENAME), &self.output_dir.join(source_site_dir)).unwrap_or_else(|| PathBuf::from(STYLES_CSS_FILENAME));
        
        // 生成更友好的章节标题
        let raw_title = source_site_path.file_stem().unwrap_or_default().to_string_lossy();
        let title = if raw_title.is_empty() || raw_title == "unknown" {
            format!("第 {} 章", spine_index + 1)
        } else {
            raw_title.to_string()
        };
        
        let final_html = TEMPLATE_HTML
            .replace("{{ title }}", &title)
            .replace("{{ body }}", &rewritten_body_str)
            .replace("{{ nav }}", &nav_html)
            .replace("{{ styles_path }}", &styles_rel_path.to_string_lossy());

        fs::write(self.output_dir.join(&source_site_path), final_html)?;
        Ok(())
    }

    /// Creates the settings for lol_html to rewrite links and structure.
    fn get_html_rewrite_settings(&self, source_epub_path: &Path, source_site_path: &Path) -> RewriteStrSettings {
        let path_map = self.path_map.clone();
        let source_epub_dir = source_epub_path.parent().unwrap_or_else(|| Path::new("")).to_path_buf();
        let source_site_path_clone = source_site_path.to_path_buf();

        RewriteStrSettings {
            element_content_handlers: vec![
                // Remove the original title, we'll use our template's title.
                element!("title", |el| { el.remove(); Ok(()) }),

                // Rewrite links in <a> and <link> tags.
                element!("a[href], link[href]", {
                    let path_map = path_map.clone();
                    let source_epub_dir = source_epub_dir.clone();
                    let source_site_path_clone = source_site_path_clone.clone();
                    move |el: &mut Element| {
                        if let Some(href) = el.get_attribute("href") {
                            if let Some(new_link) = rewrite_link(&href, &source_epub_dir, &source_site_path_clone, &path_map) {
                                el.set_attribute("href", &new_link)?;
                            }
                        }
                        Ok(())
                    }
                }),
                
                // Rewrite sources in <img>, <audio>, <video>, etc.
                element!("img[src], audio[src], video[src], source[src]", {
                    let path_map = path_map.clone();
                    let source_epub_dir = source_epub_dir.clone();
                    let source_site_path_clone = source_site_path_clone.clone();
                    move |el: &mut Element| {
                        if let Some(src) = el.get_attribute("src") {
                            if let Some(new_link) = rewrite_link(&src, &source_epub_dir, &source_site_path_clone, &path_map) {
                                el.set_attribute("src", &new_link)?;
                            }
                        }
                        Ok(())
                    }
                }),

                // Extract only the content of the <body> tag.
                element!("body", |el| { el.remove_and_keep_content(); Ok(()) })
            ], ..RewriteStrSettings::default()
        }
    }


    fn create_nav_html(&self, spine_index: usize, current_site_path: &Path) -> String {
        if self.no_nav { return String::new(); }

        let current_dir = current_site_path.parent().unwrap_or_else(|| Path::new(""));
        let mut parts = Vec::new();
        
        if spine_index > 0 {
            if let Some(prev_item) = self.spine_items.get(spine_index - 1) {
                if let Some(target_path) = self.path_map.get(&normalize_path_key(&prev_item.href)) {
                    let rel_path = pathdiff::diff_paths(target_path, current_dir).unwrap_or_else(|| target_path.clone());
                    parts.push(format!("<a href=\"{}\" class=\"nav-prev\">« 上一章</a>", rel_path.to_string_lossy()));
                }
            }
        } else { parts.push("<span></span>".to_string()); }

        let toc_path = pathdiff::diff_paths(self.output_dir.join("index.html"), self.output_dir.join(current_dir)).unwrap_or_else(|| PathBuf::from("../index.html"));
        parts.push(format!("<a href=\"{}\" class=\"nav-toc\">📚 目录</a>", toc_path.to_string_lossy()));

        if spine_index < self.spine_items.len() - 1 {
            if let Some(next_item) = self.spine_items.get(spine_index + 1) {
                if let Some(target_path) = self.path_map.get(&normalize_path_key(&next_item.href)) {
                     let rel_path = pathdiff::diff_paths(target_path, current_dir).unwrap_or_else(|| target_path.clone());
                    parts.push(format!("<a href=\"{}\" class=\"nav-next\">下一章 »</a>", rel_path.to_string_lossy()));
                }
            }
        } else { parts.push("<span></span>".to_string()); }

        parts.join("\n")
    }
}
