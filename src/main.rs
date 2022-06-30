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
    //
    //  Drawing manually seems a lot easier!
    let tileset = load_texture("assets/dejavu10x10_gs_tc.png").await.unwrap();
    let player_tile = Rect::new(0., 10.0, 10., 10.);

    loop {
        clear_background(BLACK);
        draw_texture_ex(
            tileset,
            10.,
            10.,
            WHITE,
            DrawTextureParams {
                flip_y: true,
                source: Some(player_tile),
                ..Default::default()
            },
        );

        next_frame().await
    }
}
