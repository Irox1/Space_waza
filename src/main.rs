use std::os::windows::io::HandleOrInvalid;

use minifb::{Key, Window, WindowOptions};

struct Vaisseau {
    x : u32,
    y : u32,
    dx : u32,
    dy : u32,
    width : u32,
    height : u32,
}

impl Vaisseau {
    pub fn draw(&self, buf : &mut Vec<u32>, width_screen: &u32) {
        draw_rect(buf, self.x, self.y, width_screen, self.width, self.height, 0x000000);
        draw_rect(buf, self.x, self.y, width_screen, self.width, self.height, 0xFF0000);
    }
    pub fn update(&self) {
        
    }
    
}

fn draw_pixel(buf: &mut Vec<u32>, x: u32, y: u32, width: &u32, colour : u32) {
    // Calculer l'index uniquement si (x, y) est dans les limites du buffer
    if x < *width && y * width < buf.len() as u32 {
        let index = (y * width + x) as usize;
        buf[index] = colour; // Couleur noire
    }
}

fn draw_rect(buf : &mut Vec<u32>, x : u32, y : u32, width_screen: &u32, width : u32, height : u32, colour : u32){
    for pos_y in y..(y+height){
        for pos_x in x..(x+width){
            draw_pixel(buf, pos_x, pos_y, width_screen, colour);
        }
    }
}


fn main() {
    // Dimensions de la fenêtre
    let width: usize = 640;
    let height = 360;

    // Création du framebuffer avec une couleur fixe (rouge ici)
    let buffer: Vec<u32> = vec![0xFF0000; width * height]; // 0xFF0000 = rouge

    // Création de la fenêtre
    let mut window = Window::new(
        "Exemple minifb - Cliquez pour quitter",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let real_width = width as u32;

    // Limite d'actualisation à 60 FPS
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // Boucle principale
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Mettre à jour le contenu de la fenêtre avec le framebuffer
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}

