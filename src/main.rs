#![allow(dead_code)]

extern crate rand;
extern crate piston_window;
mod postac;

use std::collections::{HashMap, HashSet};

use rand::*;
use piston_window::*;

use postac::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("D&D", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut glyphs = Glyphs::new(
        "assets/FiraSans-Regular.ttf",
        window.factory.clone(),
        TextureSettings::new(),
    ).unwrap();
    let postac = Postac::new(
        "Ao Dojo",
        (0, 0, 12, 0),
        Klasa::new("Paladyn".into(), 10, vec![1]),
        Rasa { nazwa: "Dragonborn".into() },
        Charakter::PrawyDobry,
        "Bahamut",
        Wyglad {
            rozmiar: Rozmiar::Sredni,
            wiek: 30,
            plec: Plec::Mezczyzna,
            wzrost: 210,
            waga: 140,
            oczy: "szare".into(),
            wlosy: None,
            skora: "granatowe luski".into(),
        },
        Atrybuty::podst(),
        HashMap::new(),
        HashSet::new(),
    );
    println!("Witaj, {:?}!", postac);
    println!("Twoja siła to: {}", postac.podstawowe_atrybuty.sila);
    println!(
        "Twój modyfikator siły to: {}",
        modyfikator(postac.podstawowe_atrybuty).sila
    );
    let mut rng = thread_rng();
    for i in 0..4 {
        let rzut = rng.gen_range::<u8>(1, 21);
        println!("Rzut {}.: {}", i + 1, rzut);
    }
    let mut new_game_pressed = false;
    let mut left_pressed = false;

    while let Some(e) = window.next() {
        e.press(|b| {
            match b {
                Button::Mouse(MouseButton::Left) => left_pressed = true,
                Button::Mouse(MouseButton::Right) => println!("zycze milego dnia, takze spada"),
                _ => (),
            }
        });
        e.release(|b| {
            match b {
                Button::Mouse(MouseButton::Left) => left_pressed = false,
                _ => (),
            }
        });
        e.mouse_cursor(|x, y| {
            let (w, h) = window.draw_size().into();
            let (w, h) = (w as f64, h as f64);
            new_game_pressed = 
                left_pressed && 
                x > (w / 4.) && x < (w * 3. / 4.) &&
                y > (h / 4.) && y < (h * 2. / 4.);
        });
        window.draw_2d(&e, |c, g| {
            let size = c.viewport.unwrap().window_size;
            let (x, y) = (size[0] as f64, size[1] as f64);
            clear([0., 0., 0., 1.], g);
            let color = if !new_game_pressed {
                [0., 0., 0.7, 1.]
            } else {
                [0., 0.4, 0.4, 1.]
            };
            rectangle(
                color,
                [x / 4., y / 4., x / 2., y / 4.],
                c.transform,
                g,
            );
            text::Text::new_color([1., 1., 1., 1.], (x / 30.) as _).draw(
                "New game",
                &mut glyphs,
                &c.draw_state,
                c.transform.trans(x * 3. / 8., y * 3. / 8.),
                g
            ).unwrap();
        });
    }
}
