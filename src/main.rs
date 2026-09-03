// A File System Hierarchy
//
// A classic example of the composite pattern is a computer file system where both File (Leaf) and
// Directory (Composite) can display their contents uniformly.

// ====================== //
// 1. The Component Trait //
// ====================== //

trait FileSystemItem {
    fn print(&self, indent: &str);
}

// ================ //
// 2. The Leaf Node //
// ================ //

struct File {
    name: String,
}

impl File {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl FileSystemItem for File {
    fn print(&self, indent: &str) {
        println!("{}{}", indent, self.name);
    }
}

// ===================== //
// 3. The Composite Node //
// ===================== //

struct Directory {
    name: String,
    // We use Box<dyn FileSystemItem> to store different types in the same vector
    children: Vec<Box<dyn FileSystemItem>>,
}

impl Directory {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            children: Vec::new(),
        }
    }

    fn add(&mut self, item: Box<dyn FileSystemItem>) {
        self.children.push(item);
    }
}

impl FileSystemItem for Directory {
    fn print(&self, indent: &str) {
        // Print the directory name itself
        println!("{}{}/", indent, self.name);

        // Recursively print all child elements with an increased indentation
        let new_indent = format!("{}  ", indent);
        for child in &self.children {
            child.print(&new_indent);
        }
    }
}

// ===== //
// Usage //
// ===== //

fn main() {
    // Construct individual leaf nodes
    let cargo = Box::new(File::new("Cargo.toml"));
    let main = Box::new(File::new("main.rs"));
    let readme = Box::new(File::new("README.md"));

    // Build the structural composite hierarchy
    let mut src_dir = Directory::new("src");
    src_dir.add(main);

    let mut prj_root_dir = Directory::new("project");
    prj_root_dir.add(cargo);
    prj_root_dir.add(readme);
    prj_root_dir.add(Box::new(src_dir)); // Nesting a composite into a composite!!!

    // Execute the entry method uniformly over the whole tree structure
    prj_root_dir.print("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_creation() {
        // 📄 test.txt
        let _f = File::new("test.txt");
    }

    #[test]
    fn directory_nesting() {
        // root/
        // └── dir/
        //     └── a.txt
        let mut d = Directory::new("dir");
        d.add(Box::new(File::new("a.txt")));
        let mut root = Directory::new("root");
        root.add(Box::new(d));
    }

    #[test]
    fn empty_directory() {
        // 📁 empty/
        let _d = Directory::new("empty");
    }

    #[test]
    fn single_file_in_directory() {
        // dir/
        // └── file.txt
        let mut d = Directory::new("dir");
        d.add(Box::new(File::new("file.txt")));
    }

    #[test]
    fn directory_with_multiple_files() {
        // dir/
        // ├── a.txt
        // ├── b.txt
        // └── c.txt
        let mut d = Directory::new("dir");
        d.add(Box::new(File::new("a.txt")));
        d.add(Box::new(File::new("b.txt")));
        d.add(Box::new(File::new("c.txt")));
    }

    #[test]
    fn nested_directory_one_level() {
        // outer/
        // └── inner/
        //     └── inner_file.txt
        let mut inner = Directory::new("inner");
        inner.add(Box::new(File::new("inner_file.txt")));
        let mut outer = Directory::new("outer");
        outer.add(Box::new(inner));
    }

    #[test]
    fn deep_nesting_three_levels() {
        // level1/
        // └── level2/
        //     └── level3/
        //         └── deep.txt
        let mut level3 = Directory::new("level3");
        level3.add(Box::new(File::new("deep.txt")));
        let mut level2 = Directory::new("level2");
        level2.add(Box::new(level3));
        let mut level1 = Directory::new("level1");
        level1.add(Box::new(level2));
    }

    #[test]
    fn directory_with_mixed_children() {
        // mixed/
        // ├── file.txt
        // └── sub/
        //     └── sub_file.txt
        let mut d = Directory::new("mixed");
        d.add(Box::new(File::new("file.txt")));
        let mut sub = Directory::new("sub");
        sub.add(Box::new(File::new("sub_file.txt")));
        d.add(Box::new(sub));
    }

    #[test]
    fn multiple_sibling_directories() {
        // root/
        // ├── d1/
        // │   └── f1.txt
        // └── d2/
        //     └── f2.txt
        let mut root = Directory::new("root");
        let mut d1 = Directory::new("d1");
        d1.add(Box::new(File::new("f1.txt")));
        let mut d2 = Directory::new("d2");
        d2.add(Box::new(File::new("f2.txt")));
        root.add(Box::new(d1));
        root.add(Box::new(d2));
    }

    #[test]
    fn complex_tree() {
        // root/
        // ├── root_file.txt
        // └── src/
        //     ├── main.rs
        //     └── tests/
        //         └── test.rs
        let mut root = Directory::new("root");
        root.add(Box::new(File::new("root_file.txt")));
        let mut src = Directory::new("src");
        src.add(Box::new(File::new("main.rs")));
        let mut tests = Directory::new("tests");
        tests.add(Box::new(File::new("test.rs")));
        src.add(Box::new(tests));
        root.add(Box::new(src));
    }

    #[test]
    fn very_deep_nesting_five_levels() {
        // d1/
        // └── d2/
        //     └── d3/
        //         └── d4/
        //             └── d5/
        //                 └── deepest.txt
        let mut d5 = Directory::new("d5");
        d5.add(Box::new(File::new("deepest.txt")));
        let mut d4 = Directory::new("d4");
        d4.add(Box::new(d5));
        let mut d3 = Directory::new("d3");
        d3.add(Box::new(d4));
        let mut d2 = Directory::new("d2");
        d2.add(Box::new(d3));
        let mut d1 = Directory::new("d1");
        d1.add(Box::new(d2));
    }

    #[test]
    fn large_sibling_set() {
        // root/
        // ├── file0.txt
        // ├── file1.txt
        // ...
        // └── file9.txt
        let mut root = Directory::new("root");
        for i in 0..10 {
            root.add(Box::new(File::new(&format!("file{}.txt", i))));
        }
    }
}
