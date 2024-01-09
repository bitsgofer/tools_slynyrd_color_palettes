use iced::executor;
use iced::mouse;
use iced::widget::canvas::{self, stroke, Canvas, Frame, Geometry, LineCap, Path, Stroke};
use iced::widget::{column, container, text};
use iced::{
    Application, Command, Element, Length, Point, Rectangle, Renderer, Settings, Size, Theme,
};
use palette::{self, convert::FromColor, rgb::Rgb, Hsv, Lighten, RgbHue, Saturate, ShiftHue};

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

    /// Differences in saturation for colors in the base ramp. Follow this rule:
    ///
    /// - delta[0].saturation = color[0].saturation - color[0+1].saturation
    /// - delta[middle-1].saturation = color[middle-1].saturation - color[middle].saturation
    /// - delta[middle].saturation = 0
    /// - delta[middle+1].saturation = color[middle+1].saturation - color[middle].saturation
    /// - delta[N].saturation = color[N].saturation - color[N-1].saturation
    base_ramp_saturation_deltas: Vec<f32>,

    /// Differences in brightness for colors in the base ramp, w.r.t base color.
    ///
    /// - delta[0].brightness = color[0].brightness - color[0+1].brightness
    /// - delta[middle-1].brightness = color[middle-1].brightness - color[middle].brightness
    /// - delta[middle].brightness = 0
    /// - delta[middle+1].brightness = color[middle+1].brightness - color[middle].brightness
    /// - delta[N].brightness = color[N].brightness - color[N-1].brightness
    base_ramp_brightness_deltas: Vec<f32>,
}

struct PaletteGenerator {
    cfg: Config,
    // A cached drawing that only redraw on dimension changes OR when asked.
    canvas_cache: canvas::Cache,
    ramps: Vec<Vec<Hsv>>,
}

impl Default for PaletteGenerator {
    fn default() -> PaletteGenerator {
        let hsv = hsv8_to_hsv(180.0, 87.0, 70.0).unwrap();

        PaletteGenerator {
            cfg: Config {
                hsv_base: hsv,
                colors_per_ramp: 9,
                ramps_per_palette: 8,
                base_ramp_saturation_deltas: vec![
                    -17.0, -20.0, -11.0, -05.0, 00.0, -15.0, -15.0, -15.0, -15.0,
                ],
                base_ramp_brightness_deltas: vec![
                    -14.0, -14.0, -16.0, -16.0, 00.0, 010.0, 010.0, 005.0, 005.0,
                ],
            },
            canvas_cache: canvas::Cache::default(),
            ramps: vec![],
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
        let hsv: Hsv = self.cfg.hsv_base.into_format();
        let rgb = Rgb::from_color(hsv);
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
            let pad = 2.0;

            /*
            let thin_stroke = || -> Stroke {
                Stroke {
                    width: 1.0,
                    style: stroke::Style::Solid(iced::Color::BLACK),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };
            */

            let ramps = generate_ramps(
                self.cfg.hsv_base,
                self.cfg.colors_per_ramp as usize,
                &self.cfg.base_ramp_saturation_deltas,
                &self.cfg.base_ramp_brightness_deltas,
                self.cfg.ramps_per_palette as usize,
            );

            let mut anchor = Point { x: 0.0, y: 0.0 };

            let height = frame.height() / (self.cfg.ramps_per_palette as f32 + pad);
            for (_, ramp) in ramps.iter().enumerate() {
                let n = ramp.len();
                let width = frame.width() / (n as f32 + pad);

                let box_size = Size { width, height };

                anchor.x = 0.0;
                for j in 0..n {
                    let hsv = ramp[j];
                    let rgb = Rgb::from_color(hsv);
                    // let (red, green, blue) = rgb8_from_rgb(&rgb);
                    // let (hue, saturation, value) = hsv8_from_hsv(&hsv);
                    // println!(
                    //     "draw: H={}, S= {}, V= {} ||| R={}, G= {}, B= {}",
                    //     red, green, blue, hue, saturation, value
                    // );

                    let color = iced::Color::from_rgb(rgb.red, rgb.green, rgb.blue);

                    frame.fill_rectangle(anchor, box_size, color);

                    anchor.x += width + pad;
                }
                anchor.y += height + pad;
            }
        });

        vec![rec]
    }
}

pub fn main() -> iced::Result {
    PaletteGenerator::run(Settings::default())
}

/// Generate hue-shifted ramps from given config.
fn generate_ramps(
    hsv_base: Hsv,
    colors_per_ramp: usize,
    saturation_deltas: &Vec<f32>,
    brightness_deltas: &Vec<f32>,
    ramps_per_palette: usize,
) -> Vec<Vec<Hsv>> {
    let mid = colors_per_ramp / 2;

    // calculate deltas w.r.t base color
    let mut abs_saturation_deltas: Vec<f32> = vec![0.0; colors_per_ramp];
    let mut abs_brightness_deltas: Vec<f32> = vec![0.0; colors_per_ramp];
    let mut abs_hue_deltas: Vec<f32> = vec![0.0; colors_per_ramp];
    let hue_step_per_ramp = 20.0 as f32;
    for i in (0..mid).rev() {
        abs_saturation_deltas[i] = abs_saturation_deltas[i + 1] + saturation_deltas[i];
        abs_brightness_deltas[i] = abs_brightness_deltas[i + 1] + brightness_deltas[i];
        abs_hue_deltas[i] = abs_hue_deltas[i + 1] + -1 as f32 * hue_step_per_ramp;
    }
    for i in (mid + 1)..colors_per_ramp {
        abs_saturation_deltas[i] = abs_saturation_deltas[i - 1] + saturation_deltas[i];
        abs_brightness_deltas[i] = abs_brightness_deltas[i - 1] + brightness_deltas[i];
        abs_hue_deltas[i] = abs_hue_deltas[i - 1] + hue_step_per_ramp;
    }

    // generate the base ramp
    let mut base_ramp = Vec::<Hsv>::new();
    for i in 0..colors_per_ramp {
        let color = hsv_base
            .shift_hue(abs_hue_deltas[i])
            .saturate_fixed(abs_saturation_deltas[i] / 100.0)
            .lighten_fixed(abs_brightness_deltas[i] / 100.0);
        base_ramp.push(color);
    }
    // also generate the de-saturated half of the base ramp
    for i in (1..(colors_per_ramp - 1)).rev() {
        let color = base_ramp[i].saturate(-70.0 / 100.0);
        base_ramp.push(color);
    }

    // generate all ramps
    let mut ramps: Vec<Vec<Hsv>> = vec![];

    // grayscale ramp
    let mut grayscale_ramp: Vec<Hsv> = vec![];
    for (_, base_ramp_color) in base_ramp.iter().enumerate() {
        let color = base_ramp_color.saturate_fixed(-100.0);
        grayscale_ramp.push(color);
    }
    ramps.push(grayscale_ramp);

    // upper half, above the base ramp
    let hue_shift_per_ramp = 360.0 / ramps_per_palette as f32;
    let base_ramp_index = ramps_per_palette / 2;
    for i in 0..base_ramp_index {
        let mut ramp: Vec<Hsv> = vec![];

        let hue_steps = base_ramp_index - i;
        let hue_shift = -1 as f32 * hue_shift_per_ramp * hue_steps as f32;
        for (_, base_ramp_color) in base_ramp.iter().enumerate() {
            let color = base_ramp_color.shift_hue(hue_shift);
            ramp.push(color);
        }

        ramps.push(ramp);
    }

    // base ramp (~vertical middle)
    ramps.push(base_ramp.to_vec());

    // lower half, below the base ramp
    for i in (base_ramp_index + 1)..ramps_per_palette {
        let mut ramp: Vec<Hsv> = vec![];

        let hue_steps = i - base_ramp_index;
        let hue_shift = hue_shift_per_ramp * hue_steps as f32;
        for (_, base_ramp_color) in base_ramp.iter().enumerate() {
            let color = base_ramp_color.shift_hue(hue_shift);
            ramp.push(color);
        }

        ramps.push(ramp);
    }

    ramps
}

fn rgb8_to_rgb(red: f32, green: f32, blue: f32) -> Result<Rgb, &'static str> {
    if red < 0.0 || red > 255.0 {
        return Err("red is not in [0.0, 255.0]");
    }
    if green < 0.0 || green > 255.0 {
        return Err("green is not in [0.0, 255.0]");
    }
    if blue < 0.0 || blue > 255.0 {
        return Err("blue is not in [0.0, 255.0]");
    }

    Ok(Rgb::new(
        red as f32 / 255.0,
        green as f32 / 255.0,
        blue as f32 / 255.0,
    ))
}

fn rgb8_from_rgb(rgb: &Rgb) -> (f32, f32, f32) {
    (
        (rgb.red * 255.0).round() as f32,
        (rgb.green * 255.0).round() as f32,
        (rgb.blue * 255.0).round() as f32,
    )
}

fn hsv8_from_hsv(hsv: &Hsv) -> (f32, f32, f32) {
    (
        hsv.hue.into_positive_degrees() as f32,
        (hsv.saturation * 100.0).round() as f32,
        (hsv.value * 100.0).round() as f32,
    )
}

fn hsv8_to_hsv(hue: f32, saturation: f32, value: f32) -> Result<Hsv, &'static str> {
    if hue < 0.0 || hue > 360.0 {
        return Err("hue is not in [0.0, 360.0]");
    }
    if saturation < 0.0 || saturation > 100.0 {
        return Err("saturation is not in [0.0, 100.0]");
    }
    if value < 0.0 || value > 100.0 {
        return Err("value is not in [0.0, 100.0]");
    }

    Ok(Hsv::new(
        RgbHue::from_degrees(hue as f32),
        saturation as f32 / 100.0,
        value as f32 / 100.0,
    ))
}
