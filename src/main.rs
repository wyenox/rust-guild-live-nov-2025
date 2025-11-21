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

struct Humano<'menu, 'lentes> {
    lentes: &'lentes Lentes,
    menu: &'menu Menu,
}

impl<'menu, 'lentes> Humano<'menu, 'lentes> {
    fn leer_menu_y_decidir(&self) -> &'menu str {
        match self.menu.platos.choose(&mut rand::rng()) {
            Some(plato) => self.lentes.leer(plato),
            None => "Aire,",
        }
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

    let humano = Humano {
        lentes: &lentes,
        menu: &menu,
    };

    let eleccion = humano.leer_menu_y_decidir();
    lentes.romper();

    println!("Me gustaria pedir \"{}\", por favor", eleccion);
}
