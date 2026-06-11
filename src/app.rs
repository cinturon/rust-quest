use crossterm::event::{KeyCode, KeyEvent};
use crate::quest::game::Quest;
use crate::tui::load_quest_list;
use crate::quest::profile::{load_profile, Profile, zone_progress, get_titles, create_profile_json};
pub enum Screen {
    MainMenu,
    QuestList,
    QuestDetails,
    ProfilePanel,
}

pub const MS_PER_TICK: u64 = 50;
 const MENU_ITEMS: &[&str] = &["Quest List", "Profile", "Quit"];

pub struct App {
    pub screen: Screen,
    pub main_menu_cursor: usize,
    pub menu_items: &'static [&'static str],
    pub quest_list: Vec<Quest>,
    pub quest_list_cursor: usize,
    pub current_quest: Option<Quest>,
    pub profile: Profile,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Result<Self, anyhow::Error> {

        create_profile_json()?;

        let mut profile = load_profile()?;
        zone_progress(&mut profile)?;
        get_titles(&mut profile)?;
        Ok(Self {
            screen: Screen::MainMenu,
            main_menu_cursor: 0,
            menu_items: MENU_ITEMS,
            quest_list: load_quest_list().unwrap_or_default(),
            quest_list_cursor: 0,
            current_quest: None,
            profile,
            should_quit: false,
        })
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Result<(), anyhow::Error> {
        match self.screen {
            Screen::MainMenu => self.handle_main_menu_key(key),
            Screen::QuestList => self.handle_quest_list_key(key),
            Screen::QuestDetails => self.handle_quest_details_key(key),
            Screen::ProfilePanel => self.handle_profile_panel_key(key),
        }
    }

    fn handle_main_menu_key(&mut self, key: KeyEvent) -> Result<(), anyhow::Error> {
        match key.code {
            KeyCode::Esc | KeyCode::Backspace => {
                self.should_quit = true;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.main_menu_cursor = self.main_menu_cursor.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.main_menu_cursor = (self.main_menu_cursor + 1).min(MENU_ITEMS.len() - 1);
            }
            KeyCode::Enter => match self.main_menu_cursor {
                0 => self.screen = Screen::QuestList,
                1 => self.screen = Screen::ProfilePanel,
                _ => self.should_quit = true,
            },
            _ => {}
        }
        Ok(())
    }

    fn handle_quest_list_key(&mut self, key: KeyEvent) -> Result<(), anyhow::Error> {
        match key.code {
            KeyCode::Esc | KeyCode::Backspace | KeyCode::Char('q') => {
                self.screen = Screen::MainMenu;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.quest_list_cursor = self.quest_list_cursor.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.quest_list_cursor = (self.quest_list_cursor + 1).min(self.quest_list.len() - 1);
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                self.current_quest = Some(self.quest_list[self.quest_list_cursor].clone());
                self.screen = Screen::QuestDetails;
            },
            _ => {}
        }
        Ok(())
    }

    fn handle_quest_details_key(&mut self, key: KeyEvent) -> Result<(), anyhow::Error> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Backspace => {
                self.screen = Screen::QuestList;
                self.current_quest = None;
            }
            _ => {}
        }
        Ok(())
    }    

    fn handle_profile_panel_key(&mut self, key: KeyEvent) -> Result<(), anyhow::Error> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Backspace => {
                self.screen = Screen::MainMenu;
            }
            _ => {}
        }
        Ok(())
    }
}
