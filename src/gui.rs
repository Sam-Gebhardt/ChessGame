
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Image},
    Graphics, Input, Result, Window,
};



pub async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    // Load the image and wait for it to finish
    // We also use '?' to handle errors like file-not-found
    let image = Image::load(&gfx, "board.png").await?;
    let king = Image::load(&gfx, "king-black.png").await?;

    gfx.clear(Color::WHITE);

    let piece = Rectangle::new(Vector::new(0.0, 0.0), Vector::new(47.0, 70.0));
    let board = Rectangle::new(Vector::new(0.0, 0.0), Vector::new(600.0, 600.0));

    gfx.draw_image(&image, board);
    gfx.draw_image(&king, piece);
    gfx.present(&window)?;
    // window.set_size(Vector::new(650.0, 650.0));


    loop {
        while let Some(_) = input.next_event().await {}
    }
}