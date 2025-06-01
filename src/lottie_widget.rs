use dotlottie_player::{Config, DotLottiePlayer, Player};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style}, // Added Color and Style
    widgets::Widget,
};

#[derive(Debug)]
pub struct LottieWidget {
    pub player: Option<DotLottiePlayer>,
    pub is_playing: bool,
    pub loop_animation: bool,
    pub current_frame_buffer: Option<Vec<u32>>,
    pub width: u32,
    pub height: u32,
}

impl LottieWidget {
    pub fn new() -> Self {
        Self {
            player: None,
            is_playing: false,
            loop_animation: false,
            current_frame_buffer: None, // Corrected from current_frame_data
            width: 0,
            height: 0,
        }
    }

    pub fn load_from_path(&mut self, path: &str) -> Result<(), String> {
        let config = Config {
            file_path: Some(path.to_string()),
            autoplay: false,
            loop_animation: self.loop_animation,
            ..Default::default()
        };

        let player_instance = DotLottiePlayer::new(config);

        // Width and height will be updated in the render method after the first frame.
        self.player = Some(player_instance);

        Ok(())
    }

    pub fn play(&mut self) -> bool {
        if let Some(player) = self.player.as_mut() {
            if player.play() {
                self.is_playing = true;
                return true;
            }
        }
        false
    }

    pub fn pause(&mut self) -> bool {
        if let Some(player) = self.player.as_mut() {
            if player.pause() {
                self.is_playing = false;
                return true;
            }
        }
        false
    }

    pub fn stop(&mut self) -> bool {
        if let Some(player) = self.player.as_mut() {
            if player.stop() {
                self.is_playing = false;
                return true;
            }
        }
        false
    }

    pub fn set_loop(&mut self, loop_val: bool) -> bool {
        self.loop_animation = loop_val;
        if let Some(player) = self.player.as_mut() {
            return player.set_loop(loop_val);
        }
        // If there's no player, we've still set the local loop_animation value,
        // which will be used if a player is loaded later. So, consider it a success.
        true
    }
}

// Implement Widget for &mut LottieWidget so it can be stateful
impl<'a> Widget for &'a mut LottieWidget {
    fn render(self, area: Rect, buf: &mut Buffer) { // `self` is `&mut LottieWidget`
        if let Some(player) = self.player.as_mut() {
            if self.is_playing {
                // Calling render on the player advances its state and prepares the frame.
                if player.render() { // player.render() returns a bool
                    if let Some(frame) = player.frame_data() { // get the actual frame data
                        self.current_frame_buffer = Some(frame.data.to_vec());
                        self.width = frame.width;
                        self.height = frame.height;
                    }
                }
            }
        }

        // Actual rendering logic
        if let Some(frame_buffer) = &self.current_frame_buffer {
            if self.width > 0 && self.height > 0 && area.width > 0 && area.height > 0 {
                for screen_y in 0..area.height {
                    for screen_x in 0..area.width {
                        // Scale screen coordinates to frame buffer coordinates
                        let sample_x = (screen_x as f32 * (self.width as f32) / (area.width as f32)).round() as u32;
                        let sample_y = (screen_y as f32 * (self.height as f32) / (area.height as f32)).round() as u32;

                        // Clamp coordinates to be within frame buffer bounds
                        let sample_x = sample_x.min(self.width - 1);
                        let sample_y = sample_y.min(self.height - 1);

                        let buffer_index = (sample_y * self.width + sample_x) as usize;

                        if buffer_index < frame_buffer.len() {
                            let pixel = frame_buffer[buffer_index];
                            let a = (pixel >> 24) & 0xFF;
                            let r = (pixel >> 16) & 0xFF;
                            let g = (pixel >> 8) & 0xFF;
                            let b = pixel & 0xFF;

                            if a > 128 { // Alpha threshold
                                let cell_color = Color::Rgb(r as u8, g as u8, b as u8);
                                let cell = buf.get_mut(area.x + screen_x, area.y + screen_y);
                                cell.set_char('█'); // Full block character
                                cell.set_fg(cell_color);
                                // Optionally set background if alpha is not 255, e.g. for semi-transparency
                                // cell.set_bg(Color::Black); // Or current terminal background
                            } else {
                                // Transparent pixel, clear the cell or leave as is
                                let cell = buf.get_mut(area.x + screen_x, area.y + screen_y);
                                cell.set_char(' '); // Set to space for transparency
                                cell.reset(); // Reset style
                            }
                        }
                    }
                }
            } else {
                // Fallback placeholder if dimensions are zero (or area is zero)
                default_placeholder_render(area, buf);
            }
        } else {
            // Fallback placeholder if no frame buffer
            default_placeholder_render(area, buf);
        }
    }
}

fn default_placeholder_render(area: Rect, buf: &mut Buffer) {
    for y in area.top()..area.bottom() {
        for x in area.left()..area.right() {
            buf.get_mut(x, y).set_char('#');
        }
    }
}
