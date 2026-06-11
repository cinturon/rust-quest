use crate::app::{App, Screen};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::Modifier;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::List;
use ratatui::widgets::ListItem;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Wrap;

pub fn render(frame: &mut Frame, app: &mut App) {
    match app.screen {
        Screen::MainMenu => render_main_menu(frame, app),
        Screen::QuestList => render_quest_list(frame, app),
        Screen::QuestDetails => render_quest_details(frame, app),
        Screen::ProfilePanel => render_profile_panel(frame, app),
    }
}

fn selected_list_item(label: &str, selected: bool) -> ListItem<'static> {
    let style = if selected {
        Style::default()
            .fg(Color::Black)
            .bg(Color::White)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Blue)
    };
    let prefix = if selected { "▶ " } else { "  " };
    ListItem::new(Line::from(Span::styled(format!("{prefix}{label}"), style)))
}

fn render_main_menu(frame: &mut Frame, app: &mut App) {
    let block = Block::default()
        .title("Main Menu")
        .border_style(Style::default().fg(Color::White));

    let items: Vec<ListItem> = app
        .menu_items
        .iter()
        .enumerate()
        .map(|(i, label)| selected_list_item(label, i == app.main_menu_cursor))
        .collect();

    frame.render_widget(List::new(items).block(block), frame.area());
}

fn render_quest_list(frame: &mut Frame, app: &mut App) {
    let quest_list_items: Vec<ListItem> = app
        .quest_list
        .iter()
        .enumerate()
        .map(|(i, quest)| selected_list_item(quest.title.as_str(), i == app.quest_list_cursor))
        .collect();

    let block = Block::default()
        .title("Quest List")
        .border_style(Style::default().fg(Color::White));

    frame.render_widget(List::new(quest_list_items).block(block), frame.area());
}

fn render_quest_details(frame: &mut Frame, app: &mut App) {
    let Some(quest) = app.current_quest.as_ref() else {
        frame.render_widget(
            Block::default()
                .title("No Quest Selected")
                .border_style(Style::default().fg(Color::White)),
            frame.area(),
        );
        return;
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(80),
        ])
        .split(frame.area());

    let quest_title_chunk = Line::from(Span::styled(
        format!("Title: {}", quest.title),
        Style::default().fg(Color::Green),
    ));

    frame.render_widget(
        Paragraph::new(quest_title_chunk)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Green)),
        chunks[0],
    );

    let quest_xp_chunk = Line::from(Span::styled(
        format!("XP: {}", quest.xp),
        Style::default().fg(Color::Green),
    ));

    frame.render_widget(
        Paragraph::new(quest_xp_chunk)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Green)),
        chunks[1],
    );

    frame.render_widget(
        Paragraph::new(quest.instructions.as_str())
            .block(Block::default().title("Instructions").borders(Borders::ALL))
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left), // left reads better than Center when wrapped
        chunks[2],
    );
}

fn render_profile_panel(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(30),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
        ])
        .split(frame.area());

    let level_chunk = Line::from(Span::styled(
        format!("Level: {}", app.profile.level.as_str()),
        Style::default().fg(Color::Green),
    ));

    frame.render_widget(
        Paragraph::new(level_chunk)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Green)),
        chunks[0],
    );

    let xp_chunk = Line::from(Span::styled(
        format!("XP: {}", app.profile.xp),
        Style::default().fg(Color::Green),
    ));

    frame.render_widget(
        Paragraph::new(xp_chunk)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Green)),
        chunks[1],
    );

    let titles_text = if app.profile.titles.is_empty() {
        "No titles yet".to_string()
    } else {
        app.profile.titles.join(", ")
    };

    frame.render_widget(
        Paragraph::new(titles_text)
            .block(Block::default().title("Titles").borders(Borders::ALL))
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left), // left reads better than Center when wrapped
        chunks[2],
    );

    let zone_progress_text = if app.profile.zone_progress.is_empty() {
        "No zone progress yet".to_string()
    } else {
        app.profile
            .zone_progress
            .iter()
            .map(|zone| {
                format!(
                    "{} - {}/{}",
                    zone.name, zone.completed_quests, zone.total_quests
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    };

    frame.render_widget(
        Paragraph::new(zone_progress_text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Green)),
        chunks[3],
    );

    let total_quests = app.quest_list.len();
    let completed_quests = app.profile.completed_quest_ids.len();
    let total_quests_chunk = Line::from(Span::styled(
        format!(
            "Quests Completed: {}/{} --- Progress: {:.2}%",
            completed_quests,
            total_quests,
            (completed_quests as f32 / total_quests as f32) * 100.0
        ),
        Style::default().fg(Color::Green),
    ));

    frame.render_widget(
        Paragraph::new(total_quests_chunk)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Green)),
        chunks[4],
    );
}
