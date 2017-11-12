use std::collections::HashMap;

#[derive(Debug)]
pub struct Klasa {
    pub nazwa: String,
    pub kosc_wytrz: u8,
}

#[derive(Debug)]
pub struct Rasa {
    pub nazwa: String,
}

#[derive(Debug)]
pub enum Rozmiar {
    Filigranowy,
    Drobny,
    Malutki,
    Maly,
    Sredni,
    Duzy,
    Wielki,
    Olbrzymi,
    Kolosalny(u8),
}

#[derive(Debug)]
pub enum Plec {
    Mezczyzna,
    Kobieta,
}

#[derive(Debug)]
pub struct Wyglad {
    pub rozmiar: Rozmiar,
    pub wiek: u16,
    pub plec: Plec,
    pub wzrost: u32,
    pub waga: u16,
    pub oczy: String,
    pub wlosy: Option<String>,
    pub skora: String,
}

#[derive(Debug)]
pub enum Charakter {
    PrawyDobry,
    PrawyNeu,
    PrawyZly,
    NeuDobry,
    Neutralny,
    NeuZly,
    ChaosDobry,
    ChaosNeu,
    ChaosZly,
}

#[derive(Debug)]
pub struct Atrybuty {
    pub sila: i8,
    pub zrecznosc: i8,
    pub budowa: i8,
    pub inteligencja: i8,
    pub roztropnosc: i8,
    pub charyzma: i8,
}

impl Atrybuty {
    pub fn podst() -> Self {
        Self {
            sila: 8,
            zrecznosc: 8,
            budowa: 8,
            inteligencja: 8,
            roztropnosc: 8,
            charyzma: 8,
        }
    }
}

#[derive(Debug)]
pub enum Atrybut {
    Sila,
    Zrecznosc,
    Budowa,
    Inteligencja,
    Roztropnosc,
    Charyzma,
}

#[derive(Debug)]
pub struct Postac {
    pub imie: String,
    pub klasa: Klasa,
    pub rasa: Rasa,
    pub charakter: Charakter,
    pub bostwo: String,
    pub wyglad: Wyglad,
    pub podstawowe_atrybuty: Atrybuty,
    pub zycie: u8,
    pub stluczenia: u8,
    pub klasa_pancerza: u8,
    pub umiejetnosci: HashMap<&'static str, Atrybut>,
}

pub fn modyfikator(atrybuty: Atrybuty) -> Atrybuty {
    Atrybuty {
        sila: (atrybuty.sila / 2).saturating_sub(5),
        zrecznosc: (atrybuty.zrecznosc / 2).saturating_sub(5),
        budowa: (atrybuty.budowa / 2).saturating_sub(5),
        inteligencja: (atrybuty.inteligencja / 2).saturating_sub(5),
        roztropnosc: (atrybuty.roztropnosc / 2).saturating_sub(5),
        charyzma: (atrybuty.charyzma / 2).saturating_sub(5),
    }
}
