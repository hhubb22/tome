// epub/model.rs
use serde::Deserialize;

// Corresponds to the top-level <container> tag in container.xml
#[derive(Debug, Deserialize)]
pub struct Container {
    // The field name `rootfiles` automatically matches the <rootfiles> tag
    pub(crate) rootfiles: Rootfiles,
}

// Corresponds to the <rootfiles> tag
#[derive(Debug, Deserialize)]
pub struct Rootfiles {
    // `#[serde(rename = "rootfile")]` tells Serde to collect all
    // "rootfile" child tags into this Vec.
    #[serde(rename = "rootfile")]
    pub(crate) rootfile: Vec<Rootfile>,
}

// Corresponds to a <rootfile> tag
#[derive(Debug, Deserialize)]
pub struct Rootfile {
    // The `@` prefix indicates an XML attribute
    #[serde(rename = "@full-path")]
    pub(crate) full_path: String,
    #[serde(rename = "@media-type")]
    pub(crate) _media_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub metadata: Metadata,
    pub manifest: Manifest,
    pub spine: Spine,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    #[serde(rename = "creator", default)]
    pub creator: Vec<Creator>,

    #[serde(rename = "title", default)]
    pub title: Vec<String>,

    #[serde(rename = "language", default)]
    pub language: Vec<String>,

    #[serde(rename = "identifier", default)]
    pub identifier: Vec<Identifier>,
}

#[derive(Debug, Deserialize)]
pub struct Creator {
    #[serde(rename = "@id", default)]
    pub(crate) _id: Option<String>,
    #[serde(rename = "$text")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Identifier {
    #[serde(rename = "@id", default)]
    pub(crate) _id: Option<String>,
    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct Manifest {
    #[serde(rename = "item", default)]
    pub items: Vec<ManifestItem>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ManifestItem {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@href")]
    pub href: String,
    #[serde(rename = "@media-type")]
    pub media_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Spine {
    #[serde(rename = "itemref", default)]
    pub item_refs: Vec<ItemRef>,
}

#[derive(Debug, Deserialize)]
pub struct ItemRef {
    #[serde(rename = "@idref")]
    pub idref: String,
}