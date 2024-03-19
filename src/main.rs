//! Jumble Helper for Mom.
//!
//! Key Ideas:
//! - Jumbles generally use words of length 5.
//! - There is a unique answer for each scrambled word in the Jumble.

mod wordmap;

use macroquad::prelude::*;
use wordmap::*;

pub const MIN_WORD_LENGTH: usize = 4;
pub const MAX_WORD_LENGTH: usize = 10;
pub const ENTRY_TEXT_SIZE: u16 = 72;
pub const RESULT_TEXT_SIZE: u16 = 48;

fn window_conf() -> Conf {
    Conf {
        window_title: "Jumble Helper".to_owned(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandOnly,
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Whether the text entry data needs to be changed.
enum EntryStatus {
    Changed,
    Unchanged,
    Quit,
}

/// Stores placement and dimensions for window components.
///
/// - `scx`, `scy`: screen center x and y values.
/// - `tcx`, `tcy`: text box center x and y values.
/// - `tlx`, `tby`: text box left x, bottom y, and top y values.
/// - `tw`, `th`: text box width (x) and height (y) values.
/// - `tw2`: half of text box width (x).
/// - `ew`: width of the current text entered by the user.
/// - `margin_y`: space between boxes and results along the y-axis.
/// - `offset_y`: amount by which text entry box is shifted upwards.
struct WindowValues {
    scx: f32,
    tcx: f32,
    tlx: f32,
    tby: f32,
    tty: f32,
    tw: f32,
    th: f32,
    margin_y: f32,
}

impl WindowValues {
    fn new(text: &str, font: Option<&Font>, size: u16) -> Self {
        let sw = screen_width();
        let sh = screen_height();
        let scx = sw / 2.0;
        let scy = sh / 2.0;
        let tdims = measure_text(text, font, size, 1.0);
        let center = get_text_center(text, font, size, 1.0, 0.0);
        let offset_y = sh / 20.0;

        let tcx = center.x;
        let tw = tdims.width;
        let th = tdims.height;
        let tlx = scx - tw / 2.0;
        let tcy = scy - offset_y;
        let tby = tcy + th / 2.0;
        let tty = tcy - th / 2.0;
        let margin_y = 0.0;

        Self {
            scx,
            tcx,
            tlx,
            tby,
            tty,
            tw,
            th,
            margin_y,
        }
    }
}

/// Draws the data entry box.
fn draw_text_box(wv: &WindowValues) {
    // Text box
    draw_rectangle_lines(wv.tlx, wv.tty, wv.tw, wv.th, 1.0, BLUE);
    // Debug circle
    draw_circle(wv.tlx, wv.tty, 3.0, BLUE);
}

/// Draws the text entered by the user, centered in text box.
///
/// Entry offset shifts text to the left to keep it centered in the text box.
///
/// In Macroquad, text is drawn from the *bottom*, as opposed to rectangles, which are
/// drawn from the *top* of the shape.
fn draw_text_entry(text: &str, font: Option<&Font>, size: u16, offset_x: f32, wv: &WindowValues) {
    let x = wv.scx - offset_x;
    let y = wv.tby;

    // Debug circle at text start
    draw_circle(x, y, 3.0, ORANGE);

    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font_size: size,
            font: font,
            ..Default::default()
        },
    );
}

/// Draws a single text result at `pos` and centered on the x-axis.
fn draw_answer_centered(text: &str, font: Option<&Font>, size: u16, wv: &WindowValues) {
    let answer_dims = measure_text(text, font, size, 1.0);
    let answer_w = answer_dims.width;
    let answer_h = answer_dims.height;

    let x = wv.tcx - answer_w / 2.0;
    let y = wv.tby + (wv.margin_y + answer_h);

    // Reference line
    draw_line(
        x - 2.0,
        y - answer_h,
        x + answer_w + 2.0,
        y - answer_h,
        1.0,
        GREEN,
    );
    // Debug circle
    draw_circle(x - 2.0, y - answer_h, 3.0, GREEN);

    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font_size: size,
            font: font,
            ..Default::default()
        },
    );
}

/// Handles keyboard input.
fn handle_keyboard_input(entry: &mut String, maxlen: usize) -> EntryStatus {
    let keys_up = get_keys_released();
    let num_keys_up = keys_up.len();

    if num_keys_up == 0 {
        return EntryStatus::Unchanged;
    }

    if keys_up.contains(&KeyCode::Backspace) {
        entry.pop();
        return EntryStatus::Changed;
    }

    if keys_up.contains(&KeyCode::Delete) {
        entry.clear();
        return EntryStatus::Changed;
    }

    if keys_up.contains(&KeyCode::Escape) {
        return EntryStatus::Quit;
    }

    let entry_len = entry.len();
    let entry_rem = maxlen - entry_len;
    let keys_to_take = entry_rem.min(num_keys_up);

    for keycode in keys_up.iter().take(keys_to_take) {
        match keycode {
            KeyCode::A => entry.push('A'),
            KeyCode::B => entry.push('B'),
            KeyCode::C => entry.push('C'),
            KeyCode::D => entry.push('D'),
            KeyCode::E => entry.push('E'),
            KeyCode::F => entry.push('F'),
            KeyCode::G => entry.push('G'),
            KeyCode::H => entry.push('H'),
            KeyCode::I => entry.push('I'),
            KeyCode::J => entry.push('J'),
            KeyCode::K => entry.push('K'),
            KeyCode::L => entry.push('L'),
            KeyCode::M => entry.push('M'),
            KeyCode::N => entry.push('N'),
            KeyCode::O => entry.push('O'),
            KeyCode::P => entry.push('P'),
            KeyCode::Q => entry.push('Q'),
            KeyCode::R => entry.push('R'),
            KeyCode::S => entry.push('S'),
            KeyCode::T => entry.push('T'),
            KeyCode::U => entry.push('U'),
            KeyCode::V => entry.push('V'),
            KeyCode::W => entry.push('W'),
            KeyCode::X => entry.push('X'),
            KeyCode::Y => entry.push('Y'),
            KeyCode::Z => entry.push('Z'),
            _ => (),
        }
    }

    EntryStatus::Changed
}

#[macroquad::main(window_conf)]
async fn main() {
    // Setup
    let font_loaded = load_ttf_font("./fonts/FiraMono-Bold.ttf").await.unwrap();
    let font = Some(&font_loaded);

    let words = include_str!("../dictionary/ENGLISH_US_4_TO_8.txt");
    let word_map = make_word_map(words);
    let max_word = "ABCDEFGH";

    // Entry and Answer (Jumble will only have one answer)
    let mut entry: String = "".into();
    let mut answer: String = "".into();
    let mut entry_offset = 0.0;

    let wv = WindowValues::new(max_word, font, ENTRY_TEXT_SIZE);

    // -------------------- //
    //      Main Loop       //
    // -------------------- //

    loop {
        // Input Handling
        let entry_status = handle_keyboard_input(&mut entry, 8);

        match entry_status {
            EntryStatus::Unchanged => (),
            EntryStatus::Changed => {
                entry_offset = measure_text(&entry, font, ENTRY_TEXT_SIZE, 1.0).width / 2.0;
                println!("Entry is now '{entry}' with offset {entry_offset}");
                let matches = word_map.find_match(&entry, MIN_WORD_LENGTH, MAX_WORD_LENGTH);
                println!("Matches: {matches:?}");
                if let Some(m) = matches {
                    if m.len() == 1 {
                        answer = m[0].clone();
                    }
                } else {
                    answer.clear();
                }
            }
            EntryStatus::Quit => {
                break;
            }
        }

        // Drawing
        draw_text_box(&wv);
        draw_text_entry(&entry, font, ENTRY_TEXT_SIZE, entry_offset, &wv);
        draw_answer_centered(&answer, font, RESULT_TEXT_SIZE, &wv);

        next_frame().await;
    }
}
