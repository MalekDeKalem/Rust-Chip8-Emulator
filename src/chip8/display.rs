use sdl2::{render::Canvas, video::Window, pixels::Color, rect::Rect};

use crate::SCALE;



pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;


pub struct Display {
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Display {
    pub fn new() -> Self {
        let new_display = Self {
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT]
        };

        new_display
    }

    pub fn clear(&mut self) {
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
    }

    pub fn set_screen(&mut self, index: usize, val: bool) {
        self.screen[index] = val;
    }


    pub fn get_screen(&self) -> &[bool] {
        &self.screen
    }

    pub fn draw_screen(&mut self, canvas: &mut Canvas<Window>) {
        // Clear canvas as black
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
    
        // Now set draw color to white, iterate through each point and see if it should be drawn
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        for (i, pixel) in self.screen.iter().enumerate() {
            if *pixel {
                // Convert our 1D array's index into a 2D (x,y) position
                let x = (i % SCREEN_WIDTH) as u32;
                let y = (i / SCREEN_WIDTH) as u32;
    
                // Draw a rectangle at (x,y), scaled up by our SCALE value
                let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
                canvas.fill_rect(rect).unwrap();
            }
        }
        canvas.present();
    }
}



