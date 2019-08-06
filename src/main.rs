use std::io;
use std::cmp::{min, max};
use termion;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::screen::AlternateScreen;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders, Gauge};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};


fn main() -> Result<(), io::Error> {
    let mut stdin = termion::async_stdin().keys();
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut percent = 0;
    terminal.clear()?;
    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Percentage(110),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            Gauge::default()
                .block(Block::default().title("Gauge").borders(Borders::ALL))
                .style(Style::default().fg(Color::Yellow))
                .percent(percent)
                .render(&mut f, chunks[0]);
        })?;

        let input = stdin.next();
        if let Some(Ok(key)) = input {
            match key {
                termion::event::Key::Char('q') => break,
                termion::event::Key::Char('p') => {
                    if percent < 100 {
                        percent+=1;
                    }
                },
                termion::event::Key::Char('m') => {
                    if percent > 0 {
                        percent-=1;
                    }
                },
                _ => (),
            }
        }
    }

    Ok(())
}
