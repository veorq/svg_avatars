use svg_avatars::{Rings, SvgAvatarBuilder};

fn main() {
    let svg = SvgAvatarBuilder::new()
        .identifier("foo")
        .rings(Rings::Five)
        .build();

    svg.save("bar.svg").unwrap();
}
