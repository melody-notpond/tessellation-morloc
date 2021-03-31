use std::cell::RefCell;
use std::rc::Rc;

type Vertex2 = (f32, f32);

pub type Polygon = Vec<Vertex2>;

// eventually add additional forms of tesselation, for now just have regular
// tiling, with values such as "3.4.6.4"
pub enum Tesselation {
    Regular(Vec<u32>),
}

// an infinite tiling with but scaled and anchored at an origin (offsets from
// the original vertex)
pub struct ScaledTesselation {
    angle: f32,
    scale: f32,
    offset: (f32, f32),
    spec: Tesselation,
}

struct RoseTree {
    node: Vertex2,
    angle: f32,
    children: Vec<Rc<RefCell<RoseTree>>>,
}

pub struct Mesh {
    pub polys: Vec<Polygon>,
    pub width: u32,
    pub height: u32,
}

fn rotate_around(p: (f32, f32), pivot: (f32, f32), angle: f32) -> (f32, f32) {
    // (x+yi)(cost+isint)=xcost-ysint+i(xsint+ycost)
    let shifted = (p.0 - pivot.0, p.1 - pivot.1);
    let (cos, sin) = (angle.cos(), angle.sin());
    let rotated = (
        shifted.0 * cos - shifted.1 * sin,
        shifted.0 * sin + shifted.1 * cos,
    );
    (rotated.0 + pivot.0, rotated.1 + pivot.1)
}

fn soft_contains(vec: &Vec<(f32, f32)>, v: (f32, f32), epsilon: f32) -> bool {
    for a in vec {
        let diff = (v.0 - a.0, v.1 - a.1);
        if -epsilon < diff.0 && diff.0 < -epsilon && -epsilon < diff.1 && diff.1 < -epsilon {
            return true;
        }
    }

    false
}

// stub
fn tile_tree(st: &ScaledTesselation, mut depth: u32) -> Rc<RefCell<RoseTree>> {
    use std::collections::VecDeque;

    let mut seen = Vec::new();
    let mut queue = VecDeque::new();
    let mut queue2 = VecDeque::new();

    let root = RoseTree {
        node: (0.0, 0.0),
        angle: 0.0,
        children: Vec::new(),
    };

    let root = Rc::new(RefCell::new(root));
    if depth == 0 {
        return root;
    }

    queue.push_back(root.clone());

    while depth > 0 {
        // Get next node
        match queue.pop_front() {
            Some(node) => {
                if soft_contains(&seen, node.borrow().node, 0.000001) {
                    println!("found {:?} again", node.borrow().node);
                    continue;
                }

                // Create child nodes
                match &st.spec {
                    Tesselation::Regular(tes) => {
                        let pivot = node.borrow().node;
                        let p = (pivot.0 + 1.0, pivot.1);
                        let a = node.borrow().angle;
                        for i in tes {
                            let angle: f32 = 180.0 * (*i as f32 - 2.0) + a;
                            let point = rotate_around(p, pivot, angle);
                            let child = Rc::new(RefCell::new(RoseTree {
                                node: point,
                                angle,
                                children: vec![]
                            }));
                            node.borrow_mut().children.push(child.clone());
                            queue2.push_back(child);
                        }
                    }
                }

                // Push new node
                seen.push(node.borrow().node);
            }

            // No node = next depth
            None => {
                use std::mem::swap;
                swap(&mut queue, &mut queue2);
                queue2.clear();
                depth -= 1;
            }
        }
    }

    // Root is available by this time
    root
}

// stub
fn boxed_tile_tree(st: &ScaledTesselation, width: f64, height: f64) -> RoseTree {
    RoseTree {
        node: st.offset,
        angle: 0.0,
        children: vec![],
    }
}

// stub
pub fn render(mesh: &Mesh, filename: &str) {
    use draw::{shape::LineBuilder, Canvas, Color, Drawing, Style, SvgRenderer};

    let mut canvas = Canvas::new(mesh.width, mesh.height);

    for poly in mesh.polys.iter() {
        let mut builder = LineBuilder::new(poly[0].0, poly[0].1);

        for p in poly.iter().skip(1) {
            builder = builder.line_to(p.0, p.1);
        }

        let line = builder.line_to(poly[0].0, poly[0].1).build();
        let drawing = Drawing::new()
            .with_shape(line)
            .with_style(Style::stroked(5, Color::black()));
        canvas.display_list.add(drawing);
    }

    draw::render::save(&canvas, filename, SvgRenderer::new()).expect("could not save file");
}

/*
-- tile a finite rectangle, yielding a 2D mesh
clip :: ScaledTesselation a -> x:Num -> y:Num -> Mesh a

render :: Mesh a -> SVG

-- find the tile that contains a point
lookupTile :: Point -> Mesh a -> (Polygon, a)

-- alter a value stored in a tile that contains a point
alterTile :: Point -> (a -> a) -> Mesh a -> Mesh a

-- alter all values for all tiles that are similar polygons to the one that contains the point
alterSimilar :: Point -> (a -> a) -> Mesh a -> Mesh a

-- rotate the entire mesh
rotate :: angle:Num -> Mesh a -> Mesh a

-- scale the entire mesh
scaleXY :: x:Num -> y:Num -> Mesh a -> Mesh a

-- map a function over the value of every tile without changing topology
mapTile :: (a -> b) -> Mesh a -> Mesh b

-- map a function over the tiles without changing topology but considering the
-- polygon shape and position
coordMapTile :: ([Vertex2] -> a -> b) -> Mesh a -> Mesh b

-- overlay one SVG on top of another
overlay :: SVG -> SVG -> SVG
 *
 *
 * */
