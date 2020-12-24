use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Image},
    Graphics, Input, Result, Window,
};

// Based on 600x600 so:
/*
a1 = [0 < 75, 0 < 75]; a2 = [75 < 150, 0 < 75]; ... ; a8 = [525 < 600, 0 < 75] 
...
h1 = [525 < 600, 0 < 75]; a2 = [525 < 600, 75 < 150]; ... ; a8 = [525 < 600, 525 < 600] 

Each piece falls within a box on the board, so when a click occurs it
should be checked with the back end to see if a piece exsists on the 
board at the location. If one does, then draw circles in the open squares.
Check if the next click is a legal move and move the piece if required.


Vector of meshes
______________
Render graphics
Get move
Make Algo move
Clear screen
Rerender graphics
______________

Create global mesh for each png
Loop through the board and update the mesh and place it on the board


*/
fn cords(x: usize) -> usize {
    // turn pixel position into an array index

    let m = match x {
        0..=75 => 0,
        76..=150 => 1,
        151..=225 => 2,
        226..=300 => 3,
        301..=375 => 4,
        376..=450 => 5,
        451..=525 => 6,
        526..=600 => 7,
        _ => 7
    };

    return m;
}




pub async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    // Load the image and wait for it to finish
    let image = Image::load(&gfx, "board.png").await?;
    let king = Image::load(&gfx, "king-black.png").await?;

    gfx.clear(Color::WHITE);

    let piece = Rectangle::new(Vector::new(0.0, 0.0), Vector::new(47.0, 70.0));
    let board = Rectangle::new(Vector::new(0.0, 0.0), Vector::new(600.0, 600.0));

    let mouse = gfx.screen_to_camera(&window, input.mouse().location());
    draw(window, gfx);

    loop {
        while let Some(_) = input.next_event().await {}
    }
}