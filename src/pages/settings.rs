use crate::{font::*, styles::*, AppErrorState, Message, Page, Settings};

use iced::{
    button, text_input, Button, Column, Element, Length, Row, Rule, Space, Text, TextInput,
};

#[derive(Clone, Debug)]
pub struct SettingsPage {
    settings: Settings,
    has_apikey: bool,
    has_changed: bool,
    prev_page: Box<Page>,
    apikey: text_input::State,
    unset_apikey: button::State,
    save: button::State,
    close: button::State,
}

impl SettingsPage {
    pub fn new(mut settings: Settings, prev_page: Page) -> SettingsPage {
        let has_apikey = settings.apikey.is_some();
        settings.apikey = None;
        SettingsPage {
            settings,
            has_apikey,
            has_changed: false,
            prev_page: Box::new(prev_page),
            apikey: text_input::State::new(),
            unset_apikey: button::State::new(),
            save: button::State::new(),
            close: button::State::new(),
        }
    }

    pub fn set_apikey(&mut self, apikey: String) {
        self.settings.apikey = Some(apikey);
        self.has_changed = true;
    }

    pub fn unset_apikey(&mut self) {
        self.has_changed = self.has_apikey;
        self.settings.apikey = None;
        self.has_apikey = false;
    }

    pub fn get_prev_page(&self) -> Page {
        *self.prev_page.clone()
    }

    pub fn view<'a>(&'a mut self, error_state: &AppErrorState) -> Element<crate::Message> {
        let empty = "".to_string();
        let mut content = Column::new()
            .padding(CONTAINER_PADDING)
            .spacing(LIST_SPACING)
            .push(Text::new("API key:").font(HELVETICA_BOLD))
            .push(
                TextInput::new(
                    &mut self.apikey,
                    if self.has_apikey {
                        "You are already logged in. If you want to log in with a different account, please insert your API key."
                    } else {
                        "API key"
                    },
                    &self.settings.apikey.as_ref().unwrap_or(&empty),
                    Message::ChangeApiKey,
                )
                .padding(4),
            )
            .push(
                Text::new(
                    "\
            Open https://smmdb.net/profile in your browser and sign in with Google. \
            Paste the API key given there in this text field.",
                )
                .size(14)
                .color(TEXT_HELP_COLOR),
            );

        if self.has_apikey {
            content = content.push(
                Button::new(&mut self.unset_apikey, Text::new("Logout"))
                    .style(DefaultButtonDangerStyle)
                    .on_press(Message::ResetApiKey),
            )
        }

        content = content.push(Space::with_height(Length::Units(24)));

        content = if let AppErrorState::Some(err) = error_state {
            content.push(Space::with_height(Length::Units(16))).push(
                Text::new(err)
                    .font(HELVETICA_BOLD)
                    .size(22)
                    .color(COLOR_DARK_RED),
            )
        } else {
            content
        };

        let mut buttons = Row::new();
        if self.has_changed {
            buttons = buttons
                .push(
                    Button::new(&mut self.save, Text::new("Save and close"))
                        .style(DefaultButtonStyle)
                        .on_press(Message::TrySaveSettings(self.settings.clone())),
                )
                .push(Space::with_width(Length::Units(12)));
        }
        buttons = buttons.push(
            Button::new(&mut self.close, Text::new("Close"))
                .style(DefaultButtonDangerStyle)
                .on_press(Message::CloseSettings),
        );

        content = content.push(Rule::horizontal(4)).push(buttons);

        Column::new()
            .push(Text::new("Settings").size(36))
            .push(content)
            .padding(CONTAINER_PADDING)
            .spacing(PAGE_SPACING)
            .into()
    }
}
