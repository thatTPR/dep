// TODO Storage

use serde::{Deserialize, Serialize};

struct Comment {
    Comments: Vec<Comment>,
    Content: String,
}
struct Issue {
    num: i32,
    Title: String,
    Comments: Vec<Comment>,
    Label: String,
    open: bool,
}
