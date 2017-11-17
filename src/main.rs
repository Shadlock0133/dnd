#![allow(dead_code)]

extern crate rand;
extern crate piston_window;
extern crate conrod;
mod postac;

use std::collections::{HashMap, HashSet};

// use rand::*;
use piston_window::*;
use piston_window::texture::UpdateTexture;
use conrod::*;

use postac::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("D&D", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut ui = UiBuilder::new([WIDTH as _, HEIGHT as _])
        .theme(Theme {
            label_color: conrod::color::WHITE,
            ..Theme::default()
        })
        .build();
    ui.fonts
        .insert_from_file("assets/FiraSans-Regular.ttf")
        .unwrap();
    let mut text_vertex_data = Vec::new();
    let (mut glyph_cache, mut text_texture_cache) = {
        const SCALE_TOLERANCE: f32 = 0.1;
        const POSITION_TOLERANCE: f32 = 0.1;
        let cache =
            conrod::text::GlyphCache::new(WIDTH, HEIGHT, SCALE_TOLERANCE, POSITION_TOLERANCE);
        let buffer_len = WIDTH as usize * HEIGHT as usize;
        let init = vec![128; buffer_len];
        let settings = TextureSettings::new();
        let factory = &mut window.factory;
        let texture = G2dTexture::from_memory_alpha(factory, &init, WIDTH, HEIGHT, &settings)
            .unwrap();
        (cache, texture)
    };
    let image_map = conrod::image::Map::new();

    widget_ids!{
        struct Ids {
            canvas,
            imie,
            poziom,
            doswiadczenie,
            kasa,
            klasa,
            rasa,
            charakter,
            bostwo,
            wyglad,
            podstawowe_atrybuty,
            zycie_max,
            zycie,
            stluczenia,
            klasa_pancerza,
            umiejetnosci,
            atuty,
            inicjatywa,
            rzuty_obronne,
            zwarcie,
            jezyki,
            bronie,
            wyposazenie,
            ekwipunek_schowany,
        }
    }
    let ids = Ids::new(ui.widget_id_generator());

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
    // println!("Witaj, {:?}!", postac);
    // println!("Twoja siła to: {}", postac.podstawowe_atrybuty.sila);
    // println!(
    //     "Twój modyfikator siły to: {}",
    //     modyfikator(postac.podstawowe_atrybuty).sila
    // );
    while let Some(event) = window.next() {
        let size = window.size();
        let (w, h) = (size.width as conrod::Scalar, size.height as conrod::Scalar);
        if let Some(event) = conrod::backend::piston::event::convert(event.clone(), w, h) {
            ui.handle_event(event);
        }

        event.update(|_| {
            let mut ui = ui.set_widgets();
            conrod::widget::Canvas::new()
                .pad(30.0)
                .set(ids.canvas, &mut ui);
            conrod::widget::Text::new(&format!("Imię: {}", postac.imie))
                .top_left_of(ids.canvas)
                .set(ids.imie, &mut ui);
            conrod::widget::Text::new(&format!("Poziom: {}", postac.poziom))
                .down_from(ids.imie, 20.0)
                .set(ids.poziom, &mut ui);
            conrod::widget::Text::new(&format!("Klasa: {}", postac.klasa.nazwa))
                .down_from(ids.poziom, 20.0)
                .set(ids.klasa, &mut ui);
        });

        window.draw_2d(&event, |context, graphics| {
            if let Some(primitives) = ui.draw_if_changed() {

                // A function used for caching glyphs to the texture cache.
                let cache_queued_glyphs = |graphics: &mut G2d,
                                           cache: &mut G2dTexture,
                                           rect: conrod::text::rt::Rect<u32>,
                                           data: &[u8]| {
                    let offset = [rect.min.x, rect.min.y];
                    let size = [rect.width(), rect.height()];
                    let format = piston_window::texture::Format::Rgba8;
                    let encoder = &mut graphics.encoder;
                    text_vertex_data.clear();
                    text_vertex_data.extend(data.iter().flat_map(|&b| vec![255, 255, 255, b]));
                    UpdateTexture::update(
                        cache,
                        encoder,
                        format,
                        &text_vertex_data[..],
                        offset,
                        size,
                    ).expect("failed to update texture")
                };

                // Specify how to get the drawable texture from the image. In this case, the image
                // *is* the texture.
                fn texture_from_image<T>(img: &T) -> &T {
                    img
                }

                // Draw the conrod `render::Primitives`.
                conrod::backend::piston::draw::primitives(
                    primitives,
                    context,
                    graphics,
                    &mut text_texture_cache,
                    &mut glyph_cache,
                    &image_map,
                    cache_queued_glyphs,
                    texture_from_image,
                );
            }
        });
    }
}
