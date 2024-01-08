# Pixel art color palette generator

This is a tool to create pixel art color palettes, similar to those mentioned in
[this tutorial from slynyrd.com](https://www.slynyrd.com/blog/2018/1/10/pixelblog-1-color-palettes),
or found on <https://lospec.com/palette-list>.

My goal is to create pixel art for games with visual similar to these:

<!-- GET URL from ![](images/male_explorer.png) -->
<table>
<tr>
<td><img src="https://github.com/bitsgofer/tools_slynyrd_color_palettes/raw/main/images/male_explorer.png" width="250px"></td>
<td><img src="https://github.com/bitsgofer/tools_slynyrd_color_palettes/raw/main/images/female_explorer_1.png" width="250px"></td>
<td><img src="https://github.com/bitsgofer/tools_slynyrd_color_palettes/raw/main/images/female_explorer_2.png" width="250px"></td>
<td><img src="https://github.com/bitsgofer/tools_slynyrd_color_palettes/raw/main/images/female_explorer_3.png" width="250px"></td>
</tr>
<tr>
<td><img src="https://github.com/bitsgofer/tools_slynyrd_color_palettes/raw/main/images/scene_1.png" width="250px"></td>
<td><img src="https://github.com/bitsgofer/tools_slynyrd_color_palettes/raw/main/images/scene_2.png" width="250px"></td>
<td><img src="https://github.com/bitsgofer/tools_slynyrd_color_palettes/raw/main/images/scene_3.png" width="250px"></td>
<td><img src="https://github.com/bitsgofer/tools_slynyrd_color_palettes/raw/main/images/scene_4.png" width="250px"></td>
</tr>
</table>

> Generated with DALL-E

To that end, I believe a rich palette using SLYNYRD's technique will be useful.
Something like this:

![](images/swatches.png)

However, generating it by hand is pretty tedious, so I wanted a small GUI.

This also serve as a learning project for Rust & one of its GUI lib, `iced`.

## Goals

- Core
  - [x] GUI on Linux
  - [x] Generate everything based on a base color.
  - [x] Use Hue-Saturation-Value (HSV/HSB) to choose the base color.
  - [x] Show 8 ramps with 9 color/ramp.
  - [ ] Show hexadecimal color codes.
- Stretch
  - [ ] Cross-platform GUI (native + web)
  - [ ] Customizable number of ramps besides the base ramp, as long as `count(ramps) * hue_shift_step = 360°`.
  - [ ] Customizable number of colors in a ramp.
  - [ ] Export palettes to .ase (Aseprite) format.

