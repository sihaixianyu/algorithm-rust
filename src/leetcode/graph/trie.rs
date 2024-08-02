const CHARSET_SIZE: usize = 26;

#[derive(Default)]
struct TrieNode {
    is_end: bool,
    children: [Option<Box<TrieNode>>; CHARSET_SIZE],
}

impl TrieNode {
    fn add_child(&mut self, ch: char) {
        let ch = (ch as usize) - ('a' as usize);
        self.children[ch] = Some(Box::new(TrieNode::default()));
    }

    fn get_child(&self, ch: char) -> Option<&Box<TrieNode>> {
        let ch = (ch as usize) - ('a' as usize);
        match &self.children[ch] {
            Some(node) => Some(node),
            _ => None,
        }
    }

    fn get_child_mut(&mut self, ch: char) -> Option<&mut Box<TrieNode>> {
        let ch = (ch as usize) - ('a' as usize);
        match &mut self.children[ch] {
            Some(node) => Some(node),
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, word: String) {
        let mut curr = &mut self.root;
        for ch in word.chars() {
            if curr.get_child_mut(ch).is_none() {
                curr.add_child(ch);
            }
            curr = curr.get_child_mut(ch).unwrap();
        }
        curr.is_end = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut curr = &self.root;
        for ch in word.chars() {
            if curr.get_child(ch).is_none() {
                return false;
            }
            curr = curr.get_child(ch).unwrap();
        }

        curr.is_end
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut curr = &self.root;
        for ch in prefix.chars() {
            if curr.get_child(ch).is_none() {
                return false;
            }
            curr = curr.get_child(ch).unwrap();
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn test_case_0() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        trie.search("apple".to_string());
        trie.search("app".to_string());
        trie.starts_with("app".to_string());
        trie.insert("app".to_string());
        trie.search("app".to_string());
    }
}
