use iced::executor;
use iced::widget::{container, text};
use iced::{Application, Command, Element, Settings, Theme};
use palette::{self, convert::FromColor, rgb::Rgb, Hsv};

/// Stores configuration to generate the palette.
struct Config {
    /// The color that will be in the horizontal middle of the base ramp.
    /// This color basically controls the rest of the palette generation.
    ///
    /// Example pick: flesh-tone, one of the main theme's colors, etc.
    base_color_hsv: Hsv,

    /// How many colors are in a single ramp. This MUST be an odd number.
    // TODO(mtong): make a Odd type for this
    // REF: https://www.reddit.com/r/rust/comments/swtrc6/is_it_possible_to_define_a_type_even_of_u32_and
    colors_per_ramp: u8,

    /// How many ramps are in the palette.
    /// If this number is odd, the base ramp will be in the vertical middle.
    /// If this number is even, the base ramp will be the top of the lower half.
    ///
    /// Ramps that are not the base are hue-shifted by (360/ramps_per_palette).
    ramps_per_palette: u8,
}

struct PaletteGenerator {
    cfg: Config,
}

impl Default for PaletteGenerator {
    fn default() -> PaletteGenerator {
        let base = Hsv::new(180.0, 87.0, 85.0);

        PaletteGenerator {
            cfg: Config {
                base_color_hsv: base,
                colors_per_ramp: 9,
                ramps_per_palette: 8,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {}

impl Application for PaletteGenerator {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (PaletteGenerator, Command<Self::Message>) {
        (PaletteGenerator::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Pixel art color palette generator, inspired by SLYNYRD.com's approaches")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let rgb = Rgb::from_color(self.cfg.base_color_hsv);
        let cfg_text = format!(
            "base color: (HSV={:?}, RGB= {:?}); {} ramps with {} colors/ramp",
            self.cfg.base_color_hsv, rgb, self.cfg.ramps_per_palette, self.cfg.colors_per_ramp
        );
        let hello = text(cfg_text);
        container(hello).into()
    }
}

pub fn main() -> iced::Result {
    PaletteGenerator::run(Settings::default())
}
