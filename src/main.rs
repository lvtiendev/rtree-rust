#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

struct Rect {
    bl: Point,
    tr: Point,
}

struct Node {
    bb: Rect,
    children: Vec<Node>,
    level: u32, // 1 is leaf, increasing upward along the tree
}

struct RTree {
    root: Node,
    min_children: u32,
    max_children: u32,
    height: u32,
}

impl RTree {
    fn insert(&self, p: Point) {
        let node = Node {
            bb: Rect { bl: p, tr: p },
            children: Vec::new(),
            level: 1,
        };
        self.insert_node(node, 1);
    }

    fn insert_node(&self, node: Node, level: u32) {}
}

fn main() {
    let tree = RTree {
        root: Node {
            bb: Rect {
                bl: Point { x: 1.0, y: 1.0 },
                tr: Point { x: 5.0, y: 5.0 },
            },
            children: Vec::new(),
            level: 1,
        },
        min_children: 2,
        max_children: 5,
        height: 1,
    };

    tree.insert(Point { x: 3.0, y: 3.0 });
}
