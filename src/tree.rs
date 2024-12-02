#[derive(Debug)]
pub enum Tree {
    Leaf {
        id: u64,
        name: String,
        contents: String,
    },
    Node {
        id: u64,
        name: String,
        children: Vec<Tree>,
    },
}

impl Tree {
    pub fn id(&self) -> u64 {
        match self {
            Tree::Leaf { id, .. } => *id,
            Tree::Node { id, .. } => *id,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Tree::Leaf { name, .. } => name,
            Tree::Node { name, .. } => name,
        }
    }

    pub fn name_mut(&mut self) -> &mut String {
        match self {
            Tree::Leaf { name, .. } => name,
            Tree::Node { name, .. } => name,
        }
    }

    pub fn get_by_id(&self, id: u64) -> Option<&Tree> {
        if self.id() == id {
            return Some(self);
        }
        match self {
            Tree::Leaf { .. } => None,
            Tree::Node { children, .. } => {
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
        match self {
            Tree::Node { children, .. } => {
                for (i, child) in children.iter_mut().enumerate() {
                    child.name_mut().insert_str(0, &format!("{i}-"));
                    child.uniquify_names();
                }
            }
            Tree::Leaf { .. } => {}
        }
    }
}
