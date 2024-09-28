use std::io::Result;

struct State {
    input: String,
    idx_chars: usize,
}

impl State {
    fn move_left(&mut self) {
        self.set_idx_chars(self.idx_chars.saturating_sub(1));
    }

    fn move_right(&mut self) {
        self.set_idx_chars(self.idx_chars.saturating_add(1));
    }

    fn set_idx_chars(&mut self, new_cursor_pos: usize) {
        self.idx_chars = new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn idx_bytes(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.idx_chars)
            .unwrap_or(self.input.len())
    }

    fn cursor(&self) -> u16 {
        self.idx_chars as u16
    }
}

fn main() -> Result<()> {
    let mut terminal = ratatui::init();

    let mut state = State {
        input: String::new(),
        idx_chars: 0,
    };

    let ret = loop {
        terminal.draw(|frame| {
            use ratatui::layout::{Constraint::Length, Layout, Position};
            use ratatui::widgets::Paragraph;

            let [help_area, input_area] =
                Layout::vertical([Length(1), Length(1)]).areas(frame.area());
            frame.render_widget(Paragraph::new("Press ESC to exit"), help_area);
            frame.render_widget(Paragraph::new(state.input.as_str()), input_area);
            frame.set_cursor_position(Position::new(input_area.x + state.cursor(), input_area.y));
        })?;

        use ratatui::crossterm::event::{read, Event::Key, KeyCode, KeyEvent, KeyEventKind::Press};

        let Key(KeyEvent {
            kind: Press, code, ..
        }) = read()?
        else {
            continue;
        };

        match code {
            KeyCode::Char(new_char) => {
                state.input.insert(state.idx_bytes(), new_char);
                state.move_right();
            }
            KeyCode::Backspace if state.idx_chars > 0 => {
                state.input = state
                    .input
                    .chars()
                    .enumerate()
                    .filter_map(|(i, c)| {
                        if i == state.idx_chars - 1 {
                            None
                        } else {
                            Some(c)
                        }
                    })
                    .collect();
                state.move_left();
            }
            KeyCode::Left => state.move_left(),
            KeyCode::Right => state.move_right(),
            KeyCode::Esc => break Ok(()),
            _ => {}
        }
    };

    ratatui::restore();
    ret
}
