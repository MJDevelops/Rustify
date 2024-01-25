use tuirealm::{
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent, KeyModifiers},
    props::{Alignment, BorderSides, BorderType, Borders, Color, Style, TextModifiers},
    tui::{
        style::Stylize,
        widgets::{Block, List, ListItem, Paragraph},
    },
    AttrValue, Attribute, Component, Event, MockComponent, NoUserEvent, Props, State, StateValue,
};

use crate::app::Msg;

pub struct PlaylistStates {
    title: String,
}

impl PlaylistStates {
    pub fn change_title(&mut self, title: String) {
        self.title = title;
    }
}

pub struct Playlist {
    pub props: Props,
    pub state: PlaylistStates,
}

#[derive(MockComponent)]
pub struct PlaylistComponent {
    pub component: Playlist,
}

impl Default for PlaylistStates {
    fn default() -> Self {
        Self {
            title: "".to_string(),
        }
    }
}

impl Default for Playlist {
    fn default() -> Self {
        Self {
            props: Props::default(),
            state: PlaylistStates::default(),
        }
    }
}

impl Playlist {
    pub fn label<S>(mut self, label: S) -> Self
    where
        S: AsRef<str>,
    {
        self.attr(
            Attribute::Title,
            AttrValue::Title((label.as_ref().to_string(), Alignment::Center)),
        );
        self
    }

    pub fn alignment(mut self, a: Alignment) -> Self {
        self.attr(Attribute::TextAlign, AttrValue::Alignment(a));
        self
    }

    pub fn foreground(mut self, c: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(c));
        self
    }

    pub fn background(mut self, c: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(c));
        self
    }

    pub fn modifiers(mut self, m: TextModifiers) -> Self {
        self.attr(Attribute::TextProps, AttrValue::TextModifiers(m));
        self
    }

    pub fn value(mut self, s: String) -> Self {
        self.attr(Attribute::Value, AttrValue::String(s));
        self
    }

    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }
}

impl MockComponent for Playlist {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::prelude::Rect) {
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            let text = self
                .props
                .get_or(Attribute::Value, AttrValue::String("default".to_string()))
                .unwrap_string();
            let alignment = self
                .props
                .get_or(Attribute::TextAlign, AttrValue::Alignment(Alignment::Left))
                .unwrap_alignment();
            let foreground = self
                .props
                .get_or(Attribute::Foreground, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let background = self
                .props
                .get_or(Attribute::Background, AttrValue::Color(Color::Cyan))
                .unwrap_color();
            let modifiers = self
                .props
                .get_or(
                    Attribute::TextProps,
                    AttrValue::TextModifiers(TextModifiers::empty()),
                )
                .unwrap_text_modifiers();
            let _focus = self
                .props
                .get_or(Attribute::Focus, AttrValue::Flag(false))
                .unwrap_flag();
            let title = self
                .props
                .get_or(
                    Attribute::Title,
                    AttrValue::Title((String::default(), Alignment::Center)),
                )
                .unwrap_title();

            frame.render_widget(
                List::new([ListItem::new("item 1"), ListItem::new("item 2")])
                    .style(Style::default().fg(foreground).add_modifier(modifiers))
                    .bg(background)
                    .block(
                        Block::default()
                            .border_type(BorderType::Thick)
                            .borders(BorderSides::ALL)
                            .border_style(Style::default().fg(Color::Black))
                            .title(title.0),
                    ),
                area,
            );
        }
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn state(&self) -> tuirealm::State {
        State::One(StateValue::String(self.state.title.clone()))
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            _ => CmdResult::None,
        }
    }
}

impl PlaylistComponent {
    pub fn new(initial_value: String) -> Self {
        Self {
            component: Playlist::default()
                .alignment(Alignment::Center)
                .foreground(Color::Black)
                .value(initial_value)
                .modifiers(TextModifiers::BOLD)
                .label("Playlist")
                .background(Color::Cyan),
        }
    }
}

impl Component<Msg, NoUserEvent> for PlaylistComponent {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let cmd = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            }) if ch.is_alphabetic() => Cmd::Submit,
            Event::Keyboard(KeyEvent {
                code: Key::Enter,
                modifiers: KeyModifiers::NONE,
            }) => return Some(Msg::PlaylistOpened),
            Event::Keyboard(KeyEvent {
                code: Key::Esc,
                modifiers: KeyModifiers::NONE,
            }) => return Some(Msg::AppClose),
            _ => Cmd::None,
        };

        match self.perform(cmd) {
            _ => None,
        }
    }
}
