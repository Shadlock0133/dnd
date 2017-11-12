use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Klasa {
    pub nazwa: String,
    pub kosc_wytrz: u8,
    pub bazowa_premia_do_ataku: Vec<u8>,
}

impl Klasa {
    pub fn new(nazwa: String, kosc_wytrz: u8, bazowa_premia_do_ataku: Vec<u8>) -> Self {
        Klasa {
            nazwa,
            kosc_wytrz,
            bazowa_premia_do_ataku,
        }
    }
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
    zrecznosc: i8,
    budowa: i8,
    inteligencja: i8,
    roztropnosc: i8,
    charyzma: i8,
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
pub struct Korzysc;

type Rzut = (u8, u8);

#[derive(Debug)]
pub struct Postac {
    imie: String,
    poziom: u8,
    doswiadczenie: u32,
    kasa: (u32, u32, u32, u32),
    klasa: Klasa,
    rasa: Rasa,
    charakter: Charakter,
    bostwo: String,
    wyglad: Wyglad,
    pub podstawowe_atrybuty: Atrybuty,
    zycie_max: u16,
    zycie: f32,
    stluczenia: u8,
    klasa_pancerza: u8,
    umiejetnosci: HashMap<String, Korzysc>,
    atuty: HashMap<String, Korzysc>,
    inicjatywa: u8,
    rzuty_obronne: (u8, u8, u8),
    zwarcie: u8,
    jezyki: HashSet<String>,
    //nazwa, waga, obrazenia, krytyk, zasieg, specjalne, opis
    bronie: HashMap<String, (f32, Rzut, (u8, u8), u8, Korzysc, String)>,
    //nazwa, waga, premia do KP, specjalne, opis
    wyposazenie: HashMap<String, (f32, u8, Korzysc, String)>,
    //nazwa, waga, ilosc
    ekwipunek_schowany: HashMap<String, (f32, u16)>,
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

impl Postac {
    pub fn new<S: Into<String>>(
        imie: S,
        kasa: (u32, u32, u32, u32),
        klasa: Klasa,
        rasa: Rasa,
        charakter: Charakter,
        bostwo: S,
        wyglad: Wyglad,
        podstawowe_atrybuty: Atrybuty,
        umiejetnosci: HashMap<String, Korzysc>,
        mut jezyki: HashSet<String>,
    ) -> Self {
        Postac {
            imie: imie.into(),
            poziom: 1,
            doswiadczenie: 0,
            kasa,
            klasa,
            rasa,
            charakter,
            bostwo: bostwo.into(),
            wyglad,
            podstawowe_atrybuty,
            zycie_max: 0,
            zycie: 0.0,
            stluczenia: 0,
            klasa_pancerza: 10,
            umiejetnosci,
            atuty: HashMap::new(),
            inicjatywa: 0,
            rzuty_obronne: (0, 0, 0),
            zwarcie: 0,
            jezyki: {
                jezyki.insert("Wspólny".into());
                jezyki
            },
            bronie: HashMap::new(),
            wyposazenie: HashMap::new(),
            ekwipunek_schowany: HashMap::new(),
        }
    }
}
