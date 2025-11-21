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
    mambos_mentales: StdRng,
    lentes: &'lentes Lentes,
    menu: &'menu Menu,
}

impl<'menu, 'lentes> Humano<'menu, 'lentes> {
    fn leer_menu_y_decidir(&mut self) -> &'menu str {
        match self.menu.platos.choose(&mut self.mambos_mentales) {
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

    let mut humano = Humano {
        mambos_mentales: StdRng::from_os_rng(),
        lentes: &lentes,
        menu: &menu,
    };

    let eleccion = humano.leer_menu_y_decidir();
    lentes.romper();

    println!("Me gustaria pedir \"{}\", por favor", eleccion);
}
