use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TreeNode {
    pub character: Option<char>,
    pub frequency: usize,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl Ord for TreeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency.cmp(&other.frequency)
    }
}

impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


pub fn dfs(node: &Option<Box<TreeNode>>, prefix: String, table: &mut std::collections::HashMap<char, String>) {

    if let Some(n) = node {
        if let Some(ch) = n.character {
            table.insert(ch, prefix.clone());
        } else {
            dfs(&n.left, format!("{}0", prefix), table);
            dfs(&n.right, format!("{}1", prefix), table);
        }
    }
}

pub fn generate_code(root: &TreeNode) -> HashMap<char, String> {
    let mut prefix_table = HashMap::new();

    dfs(&Some(Box::new(root.clone())), String::new(), &mut prefix_table);
    prefix_table
}


