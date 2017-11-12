#![allow(dead_code)]

extern crate rand;
mod postac;

use rand::*;

use postac::*;

use std::collections::HashMap;

fn main() {
    let postac = Postac {
        imie: "Ao Dojo".into(),
        klasa: Klasa {
            nazwa: "Paladyn".into(),
            kosc_wytrz: 10,
        },
        rasa: Rasa {
            nazwa: "Człowiek".into(),
        },
        charakter: Charakter::PrawyDobry,
        bostwo: "Bahamut".into(),
        wyglad: Wyglad {
            rozmiar: Rozmiar::Sredni,
            wiek: 30,
            plec: Plec::Mezczyzna,
            wzrost: 210,
            waga: 140,
            oczy: "szare".into(),
            wlosy: None,
            skora: "granatowe luski".into(),
        },
        podstawowe_atrybuty: Atrybuty::podst(),
        zycie: 10,
        stluczenia: 0,
        klasa_pancerza: 10,
        umiejetnosci: HashMap::new(),
    };
    println!("Witaj, {:?}!", postac);
    println!("Twoja siła to: {}", postac.podstawowe_atrybuty.sila);
    println!("Twój modyfikator siły to: {}", modyfikator(postac.podstawowe_atrybuty).sila);
    let mut rng = thread_rng();
    for i in 0..4 {
        let rzut = rng.gen_range::<u8>(1, 21);
        println!("Rzut {}.: {}", i + 1, rzut);
    }
}
