use quicksilver::{
    geom::{Vector, Shape},
    graphics::{Color, Font, FontStyle, Image, Background::Img},
    lifecycle::{run, Settings, State, Window, Asset},
    Result,
};

struct Game {
    title: Asset<Image>,
    mononoki_font_info: Asset<Image>,
}

impl State for Game {
    fn new() -> Result<Self> {
        let font_mononoki = "mononoki-Regular.ttf";

        let title = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Quicksilver Roguelike", &FontStyle::new(72.0, Color::BLACK))
        }));

        let mononoki_font_info = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render(
                "Mononoki ofnt by Matthias Tellen, thrms: SIL Open Font License 1.1",
                &FontStyle::new(20.0, Color::BLACK)
            )
        }));

        Ok(Self {
            title,
            mononoki_font_info,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        self.title.execute(|image| {
            window.draw(
                &image
                    .area()
                    .with_center((window.screen_size().x as i32 / 2, 40)),
                Img(&image),
            );
            Ok(())
        })?;

        Ok(())
    }
}


fn main() {
    let settings = Settings {
        ..Default::default()
    };

    run::<Game>("Quicksilver Roguelike", Vector::new(800, 600), settings);
}
