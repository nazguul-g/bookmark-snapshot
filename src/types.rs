// the project related types


pub enum BookmarkTree {
    Folder(Folder),
    Link(String)
}
pub struct Folder {
    pub name:String,
    pub folder:Vec<BookmarkTree>
}