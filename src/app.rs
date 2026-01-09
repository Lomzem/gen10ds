use std::io;
use std::time::Instant;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::dipswitch::DipSwitch;
use crate::ui;

/// Main application state
pub struct App {
    pub dipswitch: DipSwitch,
    pub animations: [SwitchAnimation; 8],
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            dipswitch: DipSwitch::new(),
            animations: [
                SwitchAnimation::new(false),
                SwitchAnimation::new(false),
                SwitchAnimation::new(false),
                SwitchAnimation::new(false),
                SwitchAnimation::new(false),
                SwitchAnimation::new(false),
                SwitchAnimation::new(false),
                SwitchAnimation::new(false),
            ],
            should_quit: false,
        }
    }

    /// Main application loop
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        let target_fps = 60;
        let frame_duration = std::time::Duration::from_millis(1000 / target_fps);
        let mut last_frame = Instant::now();

        while !self.should_quit {
            // Calculate delta time
            let now = Instant::now();
            let delta = now.duration_since(last_frame).as_secs_f64();
            last_frame = now;

            // Update animations
            self.update_animations(delta);

            // Render
            terminal.draw(|frame| ui::render(frame, self))?;

            // Handle input with timeout to maintain framerate
            let timeout = frame_duration.saturating_sub(last_frame.elapsed());
            if event::poll(timeout)? {
                self.handle_events()?;
            }
        }

        Ok(())
    }

    /// Update all animations based on delta time
    fn update_animations(&mut self, delta_time: f64) {
        for animation in &mut self.animations {
            animation.update(delta_time);
        }
    }

    /// Handle keyboard and other events
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) => self.handle_key_event(key_event),
            _ => {}
        }
        Ok(())
    }

    /// Handle key press events
    fn handle_key_event(&mut self, key: KeyEvent) {
        // Only process key press events, not release
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Char(c @ '1'..='8') => {
                let idx = (c as u8 - b'1') as usize;
                self.toggle_switch(idx);
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.reset_switches();
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    /// Toggle a switch and start its animation
    fn toggle_switch(&mut self, index: usize) {
        if index < 8 {
            self.dipswitch.toggle(index);
            let new_state = self.dipswitch.get(index);
            self.animations[index].start_animation(new_state);
        }
    }

    /// Reset all switches to default and animate them
    fn reset_switches(&mut self) {
        self.dipswitch.reset();
        for animation in &mut self.animations {
            animation.start_animation(false);
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

/// Animation state for a single DIP switch
#[derive(Debug, Clone, Copy)]
pub struct SwitchAnimation {
    pub target_state: bool,
    pub current_position: f64, // 0.0 = LEFT, 1.0 = RIGHT
    pub animating: bool,
}

impl SwitchAnimation {
    const ANIMATION_DURATION: f64 = 0.15; // 150ms
    const ANIMATION_SPEED: f64 = 1.0 / Self::ANIMATION_DURATION;

    pub fn new(initial_state: bool) -> Self {
        Self {
            target_state: initial_state,
            current_position: if initial_state { 1.0 } else { 0.0 },
            animating: false,
        }
    }

    /// Start animation to a new state
    pub fn start_animation(&mut self, new_state: bool) {
        self.target_state = new_state;
        self.animating = true;
    }

    /// Update animation based on delta time
    pub fn update(&mut self, delta_time: f64) {
        if !self.animating {
            return;
        }

        let target = if self.target_state { 1.0 } else { 0.0 };
        let direction = (target - self.current_position).signum();

        self.current_position += direction * Self::ANIMATION_SPEED * delta_time;

        // Clamp and check if animation complete
        if direction > 0.0 && self.current_position >= target {
            self.current_position = target;
            self.animating = false;
        } else if direction < 0.0 && self.current_position <= target {
            self.current_position = target;
            self.animating = false;
        }
    }

    /// Get current position (0.0 to 1.0)
    pub fn position(&self) -> f64 {
        self.current_position
    }
}
