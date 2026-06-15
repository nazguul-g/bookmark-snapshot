pub struct Browser {
    pub name:String,
    pub bookmark_path:String,
    pub store_type:BookmarkStoreType
}
pub enum BookmarkStoreType {
    JSON,
    SQLite
}


