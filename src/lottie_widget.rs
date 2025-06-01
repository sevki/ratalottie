use dotlottie_rs::{Config, DotLottiePlayer, Mode}; // Updated to use dotlottie-rs and add Mode
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color, // Style removed, Color kept
    widgets::Widget,
};

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
            autoplay: false, // Explicitly false, play() will be called by user
            mode: Mode::Forward, // Default, can be configured later
            loop_animation: self.loop_animation, // Set from widget state
            speed: 1.0, // Default
            use_frame_interpolation: true, // Default
            segment: vec![], // Default
            background_color: 0x00000000, // Transparent, can be configurable
            layout: Default::default(), // Default layout
            marker: String::new(), // This field exists in dotlottie-rs::Config
            // theme_id, animation_id, state_machine_id removed as they don't exist
        };

        let player_instance = DotLottiePlayer::new(config); // mut removed

        // Use a placeholder initial size for loading.
        // The actual animation dimensions will be determined after loading or during rendering.
        // These dimensions might also be used by the player to determine initial render canvas size.
        let initial_load_width = 100; // Or a sensible default / passed in
        let initial_load_height = 100; // Or a sensible default / passed in

        if !player_instance.load_animation_path(path, initial_load_width, initial_load_height) {
            return Err(format!("Failed to load animation from path: {}", path));
        }

        self.player = Some(player_instance);
        self.is_playing = false; // Reset is_playing status on new load

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

    pub fn set_loop(&mut self, loop_val: bool) {
        self.loop_animation = loop_val;
        if let Some(player) = &self.player { // player is Option<DotLottiePlayer>
            // Get a copy of current config, modify it, and set it back
            // This assumes player.config() returns a Config struct (by value or clone)
            // and player.set_config() takes a Config struct (by value).
            let mut new_config = player.config();
            new_config.loop_animation = loop_val;
            player.set_config(new_config);
        }
    }
}

// Implement Widget for &mut LottieWidget so it can be stateful
impl<'a> Widget for &'a mut LottieWidget {
    fn render(self, area: Rect, buf: &mut Buffer) { // `self` is `&mut LottieWidget`
        // Part 1: Update widget state from player
        if let Some(player) = &self.player { // player is &DotLottiePlayer
            if self.is_playing {
                let next_frame_no = player.request_frame(); // Get the next frame number
                player.set_frame(next_frame_no);      // Seek to that frame
                player.render();                      // Render it to the player's internal buffer
            }

            // Temporary workaround for missing player.size()
            self.width = 100;
            self.height = 100;

            // Given player.buffer() and buffer_len() are also missing or incompatible,
            // we cannot get frame data. Set current_frame_buffer to None.
            // The rendering part will then use a placeholder.
            self.current_frame_buffer = None;

        } else {
            // No player, ensure local buffer is also None and render placeholder
            self.current_frame_buffer = None;
            // Set width/height to 0 to ensure 'P' placeholder logic doesn't get confused
            // with the 'M' placeholder if area is drawn later.
            self.width = 0;
            self.height = 0;
            self.default_placeholder_render(area, buf, 'P'); // 'P' for No Player
            return;
        }

        // Part 2: Render based on self.current_frame_buffer, self.width, self.height
        if let Some(frame_data) = &self.current_frame_buffer {
            // This block will currently not be hit often because current_frame_buffer is set to None above.
            // If it were to be hit (e.g., if we had a way to load a static frame for testing),
            // the existing logic would apply.
            if self.width == 0 || self.height == 0 {
                self.default_placeholder_render(area, buf, 'D'); // 'D' for Zero Dimension
                return;
            }

            if area.width == 0 || area.height == 0 { // Avoid division by zero if area is zero
                return;
            }

            for screen_y in 0..area.height {
                for screen_x in 0..area.width {
                    let widget_cell_x = area.x + screen_x;
                    let widget_cell_y = area.y + screen_y;

                    // Calculate sample coordinates in the animation frame
                    let sample_x = (screen_x as f32 * self.width as f32 / area.width as f32).round() as u32;
                    let sample_y = (screen_y as f32 * self.height as f32 / area.height as f32).round() as u32;

                    // Clamp coordinates to be within frame dimensions
                    let sample_x = sample_x.min(self.width.saturating_sub(1));
                    let sample_y = sample_y.min(self.height.saturating_sub(1));

                    let pixel_index = (sample_y * self.width + sample_x) as usize;

                    if pixel_index < frame_data.len() {
                        let pixel = frame_data[pixel_index];
                        let a = (pixel >> 24) & 0xFF;
                        let r = (pixel >> 16) & 0xFF;
                        let g = (pixel >> 8) & 0xFF;
                        let b = pixel & 0xFF;

                        let cell = buf.get_mut(widget_cell_x, widget_cell_y);
                        if a > 128 { // Consider pixel not too transparent
                            cell.set_char('█');
                            cell.set_fg(Color::Rgb(r as u8, g as u8, b as u8));
                        } else {
                            cell.set_char(' ');
                            cell.reset(); // Reset style for transparent parts
                        }
                    } else {
                        // Index out of bounds, render placeholder or skip
                        let cell = buf.get_mut(widget_cell_x, widget_cell_y);
                        cell.set_char('?'); // Should not happen with current logic if frame_data exists
                    }
                }
            }
        } else {
            // current_frame_buffer is None.
            // If a player existed (so self.width/height are placeholder 100x100), draw 'M'.
            // If no player existed, 'P' was already drawn and we returned.
            if self.width > 0 && self.height > 0 {
                self.default_placeholder_render(area, buf, 'M'); // 'M' for Missing Data
            }
            // If width/height are 0 (e.g. no player case handled by 'P' already), do nothing more.
        }
    }
}

// Ensure default_placeholder_render is part of LottieWidget impl
impl LottieWidget {
    // Make sure other methods like new(), load_from_path(), play(), pause(), stop(), set_loop() are here

    fn default_placeholder_render(&self, area: Rect, buf: &mut Buffer, char_to_draw: char) {
        for y_offset in 0..area.height {
            for x_offset in 0..area.width {
                let x = area.x + x_offset;
                let y = area.y + y_offset;
                if x < buf.area.width && y < buf.area.height { // Check bounds for buffer
                    buf.get_mut(x, y).set_char(char_to_draw);
                }
            }
        }
    }
}
