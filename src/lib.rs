type Vertex2 = (f32, f32);

pub type Polygon = Vec<Vertex2>;

// eventually add additional forms of tesselation, for now just have regular
// tiling, with values such as "3.4.6.4"
pub enum Tesselation {
    Regular(Vec<i64>)
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
    children: Vec<RoseTree>
}

pub struct Mesh {
    pub polys: Vec<Polygon>,
    pub width: u32,
    pub height: u32
}

// stub
fn tile_tree(st: ScaledTesselation, mut depth: i64) -> RoseTree {
    let mut tree = RoseTree {
        node: st.offset,
        children: vec![]
    };
    let mut queue: Vec<()> = vec![];

    while depth > 0 {
        depth -= 1;
    }

    tree
}

// stub
fn boxed_tile_tree(st: ScaledTesselation, width: f64, height: f64) -> RoseTree {
    RoseTree {
        node: st.offset,
        children: vec![]
    }
}

// stub
pub fn render(mesh: &Mesh, filename: &str) {
    use draw::{Canvas, Drawing, shape::LineBuilder, Style, Color, SvgRenderer};

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
