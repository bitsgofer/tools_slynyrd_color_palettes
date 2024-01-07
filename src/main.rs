use iced::executor;
use iced::mouse;
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path};
use iced::widget::{column, container, text};
use iced::{
    Application, Command, Element, Length, Point, Rectangle, Renderer, Settings, Size, Theme,
};
use palette::{self, convert::FromColor, rgb::Rgb, Hsv};

/// Stores configuration to generate the palette.
struct Config {
    /// The color that will be in the horizontal middle of the base ramp.
    /// This color basically controls the rest of the palette generation.
    ///
    /// Example pick: flesh-tone, one of the main theme's colors, etc.
    hsv_base: Hsv,

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
    // A cached drawing that only redraw on dimension changes OR when asked.
    canvas_cache: canvas::Cache,
}

impl Default for PaletteGenerator {
    fn default() -> PaletteGenerator {
        let hsv = Hsv::new(180.0, 87.0, 85.0);

        PaletteGenerator {
            cfg: Config {
                hsv_base: hsv,
                colors_per_ramp: 9,
                ramps_per_palette: 8,
            },
            canvas_cache: canvas::Cache::default(),
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
        let rgb = Rgb::from_color(self.cfg.hsv_base);
        let cfg_text = format!(
            "base color: (HSV={:?}, RGB= {:?}); {} ramps with {} colors/ramp",
            self.cfg.hsv_base, rgb, self.cfg.ramps_per_palette, self.cfg.colors_per_ramp
        );
        let hello: text::Text = text(cfg_text);

        let cv: Canvas<&PaletteGenerator, Message> =
            Canvas::new(self).width(Length::Fill).height(Length::Fill);

        column![hello, cv,].padding(10).spacing(10).into()
    }
}

impl<Message> canvas::Program<Message, Renderer> for PaletteGenerator {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let rec = self.canvas_cache.draw(renderer, bounds.size(), |frame| {
            let pad = 20.0;

            let box_size = Size {
                width: frame.width() / 2.0 as f32,
                height: frame.height() / 2.0 - pad,
            };

            let anchor = Point { x: 0.0, y: 0.0 };

            let rgb = Rgb::from_color(self.cfg.hsv_base);
            let color = iced::Color::from_rgb(rgb.red, rgb.green, rgb.blue);

            frame.fill_rectangle(anchor, box_size, color);
        });

        vec![rec]
    }
}

pub fn main() -> iced::Result {
    PaletteGenerator::run(Settings::default())
}
