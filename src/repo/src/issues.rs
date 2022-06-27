use std::fmt::{LowerHex, UpperHex};

use serde::{Deserialize, Serialize};

struct Comment {
    Comments: Vec<Comment>,
    Content: String,
}
struct Label {
    id: String,
    Description: String,
    color: String, // Hex
}
struct Issue {
    num: i32,
    Title: String,
    Comments: Vec<Comment>,
    Label: &Label,
    open: bool,
}
// TODO HP
struct Issues {
    issues: Vec<Issues>,
    Labels: Vec<Label>,
}
