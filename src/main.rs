use tesselation_morloc::*;

fn main() {
    let tes = Tesselation::Regular(vec![3, 4, 6, 4]);
    let scaled = ScaledTesselation {
        angle: 0.0,
        scale: 1.0,
        offset: (0.0, 0.0),
        spec: tes,
    };

    let tree = tile_tree(&scaled, 5);
    render_tree(tree, "uwu.svg");
}
