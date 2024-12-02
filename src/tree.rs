#[derive(Debug)]
pub enum Contents {
    Leaf(String),
    Node(Vec<Tree>),
}

#[derive(Debug)]
pub struct Tree {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) contents: Contents,
}

impl Tree {
    pub fn get_by_id(&self, id: u64) -> Option<&Tree> {
        if self.id == id {
            return Some(self);
        }
        match &self.contents {
            Contents::Leaf(_) => None,
            Contents::Node(children) => {
                for child in children {
                    if let Some(found) = child.get_by_id(id) {
                        return Some(found);
                    }
                }
                None
            }
        }
    }

    pub fn uniquify_names(&mut self) {
        match self.contents {
            Contents::Node(ref mut children) => {
                for (i, child) in children.iter_mut().enumerate() {
                    child.name.insert_str(0, &format!("{i}-"));
                    child.uniquify_names();
                }
            }
            Contents::Leaf(_) => {}
        }
    }
}
