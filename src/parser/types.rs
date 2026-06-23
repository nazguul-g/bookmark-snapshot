use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 1. The top-level root of the JSON file
#[derive(Debug, Serialize, Deserialize)]
pub struct ChromiumBookmarks {
    pub checksum: String,
    pub roots: BookmarkRoots,
    pub version: i32,
}

// 2. The roots object contains these exact three entry points
#[derive(Debug, Serialize, Deserialize)]
pub struct BookmarkRoots {
    pub bookmark_bar: BookmarkNode,
    pub other: BookmarkNode,
    pub synced: BookmarkNode,
}

// 3. This is the core recursive structure. 
// Both folders and bookmarks use this exact same struct shape.
#[derive(Debug, Serialize, Deserialize)]
pub struct BookmarkNode {
    pub id: String,
    pub name: String,
    pub date_added: String,
    pub date_last_used: String,
    
    // "url" or "folder"
    #[serde(rename = "type")]
    pub node_type: NodeType, 

    // Only present if node_type == NodeType::Url
    pub url: Option<String>, 

    // Only present if node_type == NodeType::Folder
    // This makes the structure recursive (a folder contains a list of nodes)
    pub children: Option<Vec<BookmarkNode>>, 
    
    pub date_modified: Option<String>, // Usually only on folders
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NodeType {
    Url,
    Folder,
}