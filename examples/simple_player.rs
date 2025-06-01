use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend, // Backend removed
    Terminal,
};
use ratalottie::LottieWidget; // Assuming your crate is named ratalottie
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create LottieWidget and load animation
    let mut lottie_widget = LottieWidget::new();
    if let Err(e) = lottie_widget.load_from_path("examples/assets/check.json") {
        eprintln!("Failed to load animation: {}", e);
        // Cleanup terminal before exiting on error
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
        return Err(Box::new(io::Error::new(io::ErrorKind::Other, e)));
    }

    lottie_widget.set_loop(true);
    lottie_widget.play();

    let mut last_frame_time = Instant::now();
    let frame_duration = Duration::from_secs_f32(1.0 / 30.0); // 30 FPS

    loop {
        terminal.draw(|f| {
            let size = f.size();
            // The LottieWidget will be drawn here.
            // Its `render` method (implemented for `&mut LottieWidget`) will be called.
            // This method now handles fetching the frame from the player if it's playing.
            f.render_widget(&mut lottie_widget, size);
        })?;

        let current_time = Instant::now();
        let elapsed = current_time.duration_since(last_frame_time);

        if elapsed < frame_duration {
            if event::poll(frame_duration - elapsed)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char(' ') => {
                            if lottie_widget.is_playing {
                                lottie_widget.pause();
                            } else {
                                lottie_widget.play();
                            }
                        }
                        _ => {}
                    }
                }
            }
        } else {
            // If processing took longer than frame_duration, poll for 0 duration (non-blocking)
             if event::poll(Duration::from_secs(0))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char(' ') => {
                            if lottie_widget.is_playing {
                                lottie_widget.pause();
                            } else {
                                lottie_widget.play();
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        last_frame_time = current_time;
        // The player's frame progression is handled by its `render` method,
        // which is called by LottieWidget's `render` method.
        // No explicit tick() call needed here if render() advances the frame.
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
