use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Search {
    pub q: String
}

