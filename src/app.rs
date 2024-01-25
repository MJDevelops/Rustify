use crate::{
    auth::{init_spotify, refresh_auth_code},
    tui::playlist::PlaylistComponent,
};
use anyhow::Result;
use rspotify::{clients::OAuthClient, AuthCodeSpotify};
use serde::Deserialize;
use std::{env, time::Duration};
use tuirealm::{
    terminal::TerminalBridge,
    tui::layout::{Constraint, Direction, Layout},
    Application, EventListenerCfg, NoUserEvent, PollStrategy, Update,
};

pub struct Envs {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Id {
    Playlist,
    Album,
    Song,
}

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    PlaylistBlur,
    PlaylistOpened,
    None,
}

#[derive(Deserialize)]
pub struct Query {
    pub code: String,
}

pub struct Model {
    pub app: Application<Id, Msg, NoUserEvent>,
    pub quit: bool,
    pub redraw: bool,
    pub terminal: TerminalBridge,
    pub spotify: AuthCodeSpotify,
}

impl Model {
    pub fn view(&mut self) {
        assert!(self
            .terminal
            .raw_mut()
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Length(3)].as_ref())
                    .split(f.size());
                self.app.view(&Id::Playlist, f, chunks[0]);
            })
            .is_ok());
    }

    pub fn new() -> Result<Self, &'static str> {
        let quit = false;
        let redraw = false;
        let terminal = if let Ok(bridge) = TerminalBridge::new() {
            bridge
        } else {
            return Err("Couldn't create terminal bridge");
        };
        let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1)),
        );
        let spotify = init_spotify();

        assert!(app
            .mount(
                Id::Playlist,
                Box::new(PlaylistComponent::new("test".to_string())),
                vec![]
            )
            .is_ok());

        assert!(app.active(&Id::Playlist).is_ok());

        Ok(Self {
            app,
            redraw,
            quit,
            terminal,
            spotify,
        })
    }

    pub async fn application_loop(&mut self) {
        // Setup tasks
        let _ = self.get_token().await;
        let _ = self.terminal.enter_alternate_screen();
        let _ = self.terminal.enable_raw_mode();

        while !self.quit {
            match self.app.tick(PollStrategy::Once) {
                Ok(messages) if messages.len() > 0 => {
                    self.redraw = true;
                    for msg in messages.into_iter() {
                        let mut msg = Some(msg);
                        while msg.is_some() {
                            msg = self.update(msg);
                        }
                    }
                }
                _ => {}
            }

            if self.redraw {
                self.view();
                self.redraw = false;
            }
        }

        let _ = self.terminal.leave_alternate_screen();
        let _ = self.terminal.disable_raw_mode();
        let _ = self.terminal.clear_screen();
    }
}

impl Update<Msg> for Model {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        if let Some(msg) = msg {
            self.redraw = true;
            match msg {
                Msg::AppClose => {
                    self.quit = true;
                    None
                }
                // TODO - Implement match arms
                Msg::PlaylistBlur => None,
                Msg::PlaylistOpened => None,
                Msg::None => None,
            }
        } else {
            None
        }
    }
}

impl Model {
    /// Loads token from cache or requests a new token and saves it internally
    pub async fn get_token(&self) -> Result<(), &'static str> {
        let token = self.spotify.read_token_cache(false).await;

        match token {
            Ok(token) => {
                if let Some(token) = token {
                    if token.is_expired() {
                        let _ = refresh_auth_code(&self.spotify, &token).await;
                        return Ok(());
                    } else {
                        *self.spotify.token.lock().await.unwrap() = Some(token);
                        return Ok(());
                    }
                } else {
                    match self.get_token_auto().await {
                        Ok(_) => return Ok(()),
                        Err(_) => return Err("Couldn't fetch token"),
                    }
                }
            }
            // Double work, fine for now
            Err(_) => match self.get_token_auto().await {
                Ok(_) => return Ok(()),
                Err(_) => return Err("Couldn't fetch token"),
            },
        }
    }

    pub async fn get_token_auto(&self) -> Result<(), &'static str> {
        // Manual auth for now, in the future auto auth should be implemented
        let url = self.spotify.get_authorize_url(true).unwrap();
        println!("Open the following link in the browser and paste in the response URL:");
        println!("{}", url);

        let mut input = "".to_string();

        if let Ok(_) = std::io::stdin().read_line(&mut input) {
            let code = self.spotify.parse_response_code(&input).unwrap();
            match self.spotify.request_token(&code).await {
                Ok(_) => return Ok(()),
                Err(_) => return Err("Error requesting token"),
            }
        } else {
            return Err("Couldn't parse URL");
        }
    }
}

impl Envs {
    pub fn new() -> Option<Self> {
        let client_id: String = if let Ok(val) = env::var("CLIENT_ID") {
            val
        } else {
            return None;
        };

        let client_secret: String = if let Ok(val) = env::var("CLIENT_SECRET") {
            val
        } else {
            return None;
        };

        Some(Self {
            client_id,
            client_secret,
        })
    }
}
