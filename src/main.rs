use macroquad::prelude::*;

#[macroquad::main("Crabby Cave")]
async fn main() {
    // TODO figure out how json tilesets work, because tldk and ogmo json files don't work,
    //  Tiled map editor tilesets do not contain the "tileset" according to macroquad-tiled.
    //  I don't understand yet why this format requires a layer, I just want the tileset!, I don't
    //  need to pre-paint layers!
    //
    //  Maybe hand craft the json, write our own code to tile the png to textures, or split the png
    //  to separate image files.
    let tileset = load_texture("assets/dejavu10x10_gs_tc.png").await.unwrap();
    let tileset_json = load_string("assets/dejavu10x10_gs_tc.tsj").await.unwrap();
    let tileset_map =
        macroquad_tiled::load_map(&tileset_json, &[("dejavu10x10_gs_tc.png", tileset)], &[])
            .unwrap();

    dbg!(&tileset_map);

    // let player = tileset_map
    //     .get_tile("dejavu10x10_gs_tc.png", 0, 10)
    //     .as_ref()
    //     .unwrap();

    loop {
        clear_background(BLACK);
        draw_texture(tileset, 0., 100., WHITE);

        next_frame().await
    }
}
