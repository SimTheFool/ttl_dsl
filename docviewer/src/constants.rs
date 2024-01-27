use once_cell::sync::Lazy;
use std::path::Path;

pub const VIEWER_WINDOW: &str = "Viewer";

// ### ~~~ INCLUDE STATIC FILENAME FOR DOCUMENT VIEWS ~~~ ### \\
macro_rules! compile_fs_views {
    ($relative_path:literal, $absolute_path:literal) => {
        HtmlViewPath {
            filename: Lazy::new(|| {
                Path::new($relative_path)
                    .parent()
                    .and_then(|filename| filename.file_name())
                    .and_then(|filename| filename.to_str())
                    .ok_or("Could not get filename")
            }),
        }
    };
}

#[iftree::include_file_tree(
    "
paths = '/ui/src/app/*/page.tsx'
template.initializer = 'compile_fs_views'
"
)]
#[derive(Debug)]
pub struct HtmlViewPath {
    filename: Lazy<Result<&'static str, &'static str>>,
}
impl HtmlViewPath {
    pub fn get_path(&self) -> Result<String, &'static str> {
        self.filename.map(|filename| format!("/{}", filename))
    }
}
// ### ~~~ END ~~~ ### \\
