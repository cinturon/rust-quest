use crate::app::{App, MS_PER_TICK};
use crate::ui;
use crate::quest::game::{load_quest_pack, Quest};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{self, Stdout};
use std::time::Duration;

pub fn run_tui() -> Result<(), anyhow::Error> {
    let mut terminal = setup_tui()?;
    let mut app = App::new()?;

    loop {
        terminal.draw(|frame| ui::render(frame, &mut app))?;

        if event::poll(Duration::from_millis(MS_PER_TICK))?
            && let Event::Key(key) = event::read()?
        {
            app.handle_key(key)?;
        }
        if app.should_quit {
            break;
        }
    }

    teardown_tui(&mut terminal)?;
    Ok(())
}

fn setup_tui() -> Result<Terminal<CrosstermBackend<Stdout>>, anyhow::Error> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    Ok(terminal)
}

fn teardown_tui(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), anyhow::Error> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}


pub fn load_quest_list() -> Result<Vec<Quest>, anyhow::Error> {
    let quest_pack = load_quest_pack()?;
    
    if quest_pack.quests.is_empty() {
        eprintln!("No quests found");
        Ok(vec![])
    }
    else {
        Ok(quest_pack.quests)
    }    
}

