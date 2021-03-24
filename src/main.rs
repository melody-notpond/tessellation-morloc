use tesselation_morloc::*;

fn main() {
    render(&Mesh {
        polys: vec![vec![(10.0, 10.0), (10.0, 90.0), (90.0, 90.0), (90.0, 10.0)]],
        width: 100,
        height: 100,
    }, "poly.svg")
}
