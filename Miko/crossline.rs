use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 400;
const HEIGHT: usize = 400;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    for i in 0..WIDTH.min(HEIGHT) {
        buffer[i * WIDTH + i] = 0xFFFFFF;
        buffer[i * WIDTH + (WIDTH - 1 - i)] = 0xFFFFFF;
    }

    let mut window = Window::new(
        "RGraphics - Cross Line",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Window creation failed: {}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
