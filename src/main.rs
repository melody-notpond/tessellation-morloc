use tesselation_morloc::*;

fn main() {
    let tes = Tesselation::Regular(vec![3, 4, 6, 4]);

    render(
        &Mesh {
            polys: vec![vec![(10.0, 10.0), (10.0, 90.0), (90.0, 90.0), (90.0, 10.0)]],
            width: 100,
            height: 100,
        },
        "poly.svg",
    )
}
