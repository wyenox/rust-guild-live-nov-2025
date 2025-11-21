use rand::prelude::*;

struct TextoChiquito(String);

impl From<&str> for TextoChiquito {
    fn from(s: &str) -> Self {
        TextoChiquito(s.to_string())
    }
}

struct Menu {
    platos: Vec<TextoChiquito>,
}

struct Lentes;

impl Lentes {
    fn leer<'lentes, 'texto>(&'lentes self, texto: &'texto TextoChiquito) -> &'texto str {
        &texto.0
    }

    fn romper(self) {}
}

fn leer_menu_y_decidir<'menu, 'lentes>(menu: &'menu Menu, lentes: &'lentes Lentes) -> &'menu str {
    match menu.platos.choose(&mut rand::rng()) {
        Some(plato) => lentes.leer(plato),
        None => "Aire,",
    }
}

fn main() {
    let lentes = Lentes;
    let menu = Menu {
        platos: vec![
            "Chivito".into(),
            "Asado".into(),
            "Panchos".into(),
            "Milanesa".into(),
            "Hamburguesa".into(),
            "Vino Tinto".into(),
        ],
    };

    let eleccion = leer_menu_y_decidir(&menu, &lentes);
    lentes.romper();
    println!("Me gustaria pedir \"{}\", por favor", eleccion);
}
