use crate::{
    emu::*,
    icon,
    pages::{InitPage, SavePage, SettingsPage},
    smmdb::{Course2Response, Difficulty, QueryParams, SmmdbUser, SortOptions},
    styles::*,
    widgets::SmmdbTab,
    EmuSave, Page, Progress, Settings, Smmdb,
};

use futures::future;
use iced::{
    button, container, executor, Application, Background, Button, Clipboard, Column, Command,
    Container, Element, Length, Row, Space, Subscription,
};
use iced_native::{keyboard, subscription, Event};
use nfd::Response;
use smmdb_lib::{CourseEntry, SavedCourse};
use std::convert::TryInto;

pub struct App {
    state: AppState,
    error_state: AppErrorState,
    settings: Settings,
    current_page: Page,
    smmdb: Smmdb,
    _window_size: WindowSize,
    settings_button: button::State,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AppState {
    Default,
    Loading,
    UploadSelect(SavedCourse),
    SwapSelect(usize),
    DownloadSelect(usize),
    DeleteSelect(usize),
    DeleteSmmdbSelect(String),
    Downloading {
        save_index: usize,
        smmdb_id: String,
        progress: f32,
    },
}

#[derive(Clone, Debug)]
pub enum AppErrorState {
    Some(String),
    None,
}

#[derive(Clone, Debug)]
pub enum Message {
    Empty,
    SetWindowSize(WindowSize),
    OpenSave(EmuSave),
    OpenCustomSave,
    LoadSave(Box<smmdb_lib::Save>, String),
    LoadSaveError(String),
    FetchSaveCourses(Vec<String>),
    FetchCourses,
    FetchSelfCourses,
    FetchError(String),
    SetSaveCourseResponse(Vec<Course2Response>),
    SetSmmdbCourses(Vec<Course2Response>),
    SetSelfSmmdbCourses(Vec<Course2Response>),
    SetSmmdbCourseThumbnail(Vec<u8>, String),
    SetSmmdbTab(SmmdbTab),
    InitUploadCourse(SavedCourse),
    UploadCourse(SavedCourse),
    UploadSucceeded(SavedCourse, String),
    InitSwapCourse(usize),
    SwapCourse(usize, usize),
    InitDownloadCourse(usize),
    DownloadCourse(usize, String),
    DownloadProgressed(Progress),
    InitDeleteCourse(usize),
    DeleteCourse(usize),
    InitDeleteSmmdbCourse(String),
    DeleteSmmdbCourse(String),
    ReloadAfterDelete(String),
    TitleChanged(String),
    UploaderChanged(String),
    DifficultyChanged(Difficulty),
    SortChanged(SortOptions),
    ApplyFilters,
    PaginateForward,
    PaginateBackward,
    PaginateSelfForward,
    PaginateSelfBackward,
    UpvoteCourse(String),
    DownvoteCourse(String),
    ResetCourseVote(String),
    SetVoteCourse(String, i32),
    OpenSettings,
    TrySaveSettings(Settings),
    SaveSettings((Settings, Option<SmmdbUser>)),
    RejectSettings(String),
    CloseSettings,
    ChangeApiKey(String),
    ResetApiKey,
    ResetState,
}

#[derive(Clone, Debug)]
pub enum WindowSize {
    S,
    M,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        let components = guess_emu_dir().unwrap();
        let settings = Settings::load().unwrap();
        let smmdb = Smmdb::new(settings.apikey.clone());
        let mut commands = vec![async move { Message::FetchCourses }.into()];
        if let Some(apikey) = &settings.apikey {
            let settings = settings.clone();
            commands.push(Command::perform(
                Smmdb::try_sign_in(apikey.clone()),
                move |res| match res {
                    Ok(user) => Message::SaveSettings((settings.clone(), Some(user))),
                    Err(err) => Message::FetchError(err),
                },
            ))
        }
        (
            App {
                state: AppState::Default,
                error_state: AppErrorState::None,
                settings,
                current_page: Page::Init(InitPage::new(components)),
                smmdb,
                _window_size: WindowSize::M,
                settings_button: button::State::new(),
            },
            Command::batch(commands),
        )
    }

    fn title(&self) -> String {
        String::from("SMMDB")
    }

    fn update(&mut self, message: Self::Message, _: &mut Clipboard) -> Command<Self::Message> {
        match message {
            Message::Empty => Command::none(),
            Message::SetWindowSize(window_size) => {
                // TODO listen to application resize somehow
                self._window_size = window_size;
                Command::none()
            }
            Message::OpenSave(save) => {
                self.state = AppState::Loading;
                let display_name = save.get_display_name().clone();
                Command::perform(
                    async move {
                        futures::join!(
                            smmdb_lib::Save::new(save.get_location().clone()),
                            future::ok::<String, String>(display_name)
                        )
                    },
                    move |res| match res {
                        (Ok(smmdb_save), Ok(display_name)) => {
                            Message::LoadSave(Box::new(smmdb_save), display_name)
                        }
                        (Err(err), _) => Message::LoadSaveError(err.into()),
                        _ => todo!(),
                    },
                )
            }
            Message::OpenCustomSave => {
                self.state = AppState::Loading;
                match nfd::open_pick_folder(None) {
                    Ok(result) => match result {
                        Response::Okay(file_path) => {
                            Command::perform(smmdb_lib::Save::new(file_path.clone()), move |res| {
                                match res {
                                    Ok(smmdb_save) => Message::LoadSave(
                                        Box::new(smmdb_save),
                                        file_path.clone().to_string_lossy().into(),
                                    ),
                                    Err(err) => Message::LoadSaveError(err.into()),
                                }
                            })
                        }
                        Response::OkayMultiple(_files) => {
                            println!("Not multifile select");
                            Command::none()
                        }
                        Response::Cancel => {
                            println!("User canceled");
                            Command::none()
                        }
                    },
                    Err(err) => async move { Message::LoadSaveError(format!("{:?}", err)) }.into(),
                }
            }
            Message::LoadSave(smmdb_save, display_name) => {
                self.state = AppState::Default;
                self.error_state = AppErrorState::None;
                self.current_page = Page::Save(Box::new(SavePage::new(
                    *smmdb_save.clone(),
                    display_name,
                    self.smmdb.get_course_responses(),
                )));
                let course_ids: Vec<String> = smmdb_save
                    .get_own_courses()
                    .iter()
                    .filter_map(|c| c.as_ref())
                    .map(|course| {
                        if let CourseEntry::SavedCourse(course) = &**course {
                            course.get_course().get_smmdb_id()
                        } else {
                            None
                        }
                    })
                    .flatten()
                    .collect();
                if course_ids.is_empty() {
                    Command::none()
                } else {
                    async move { Message::FetchSaveCourses(course_ids.clone()) }.into()
                }
            }
            Message::LoadSaveError(err) => {
                eprintln!("{}", &err);
                self.error_state =
                    AppErrorState::Some(format!("Could not load save file. Full error:\n{}", err));
                Command::none()
            }
            Message::FetchSaveCourses(course_ids) => {
                let query_params = QueryParams {
                    limit: 120,
                    ids: Some(course_ids),
                    ..QueryParams::default()
                };
                let apikey = self.settings.apikey.clone();
                Command::perform(Smmdb::update(query_params, apikey), move |res| match res {
                    Ok(courses) => Message::SetSaveCourseResponse(courses),
                    Err(err) => Message::FetchError(err.to_string()),
                })
            }
            Message::FetchCourses => {
                self.state = AppState::Loading;
                Command::perform(
                    Smmdb::update(
                        self.smmdb.get_query_params().clone(),
                        self.settings.apikey.clone(),
                    ),
                    move |res| match res {
                        Ok(courses) => Message::SetSmmdbCourses(courses),
                        Err(err) => Message::FetchError(err.to_string()),
                    },
                )
            }
            Message::FetchSelfCourses => {
                if self.settings.apikey.is_some() {
                    self.state = AppState::Loading;
                    Command::perform(
                        Smmdb::update_self(
                            self.smmdb.get_own_query_params().clone(),
                            self.settings.apikey.clone(),
                        ),
                        move |res| match res {
                            Ok(courses) => Message::SetSelfSmmdbCourses(courses),
                            Err(err) => Message::FetchError(err.to_string()),
                        },
                    )
                } else {
                    Command::none()
                }
            }
            Message::FetchError(err) => {
                eprintln!("FetchError: {}", &err);
                self.error_state = AppErrorState::Some(err);
                Command::none()
            }
            Message::SetSaveCourseResponse(courses) => {
                self.smmdb.set_courses(courses, false);
                if let Page::Save(ref mut save_page) = self.current_page {
                    save_page.set_course_response(self.smmdb.get_course_responses())
                }
                self.state = AppState::Default;
                Command::none()
            }
            Message::SetSmmdbCourses(courses) => {
                self.state = AppState::Default;
                self.error_state = AppErrorState::None;
                self.smmdb.set_courses(courses, true);
                let course_ids: Vec<String> =
                    self.smmdb.get_course_panels().keys().cloned().collect();

                let mut commands = Vec::<Command<Message>>::new();
                for id in course_ids {
                    commands.push(Command::perform(
                        async move {
                            futures::join!(Smmdb::fetch_thumbnail(id.clone()), async { id })
                        },
                        |(thumbnail, id)| {
                            if let Ok(thumbnail) = thumbnail {
                                Message::SetSmmdbCourseThumbnail(thumbnail, id)
                            } else {
                                // TODO handle error
                                Message::Empty
                            }
                        },
                    ));
                }
                Command::batch(commands)
            }
            Message::SetSelfSmmdbCourses(courses) => {
                self.state = AppState::Default;
                self.error_state = AppErrorState::None;
                self.smmdb.set_own_courses(courses, true);
                let course_ids: Vec<String> =
                    self.smmdb.get_own_course_panels().keys().cloned().collect();

                let mut commands = Vec::<Command<Message>>::new();
                for id in course_ids {
                    commands.push(Command::perform(
                        async move {
                            futures::join!(Smmdb::fetch_thumbnail(id.clone()), async { id })
                        },
                        |(thumbnail, id)| {
                            if let Ok(thumbnail) = thumbnail {
                                Message::SetSmmdbCourseThumbnail(thumbnail, id)
                            } else {
                                // TODO handle error
                                Message::Empty
                            }
                        },
                    ));
                }
                Command::batch(commands)
            }
            Message::SetSmmdbCourseThumbnail(thumbnail, id) => {
                self.smmdb.set_course_panel_thumbnail(&id, thumbnail);
                Command::none()
            }
            Message::SetSmmdbTab(tab) => {
                if let Page::Save(ref mut save_page) = self.current_page {
                    save_page.set_smmdb_tab(tab)
                }
                Command::none()
            }
            Message::InitUploadCourse(course) => {
                self.state = AppState::UploadSelect(course);
                Command::none()
            }
            Message::UploadCourse(course) => {
                self.state = AppState::Loading;
                if let Some(apikey) = &self.settings.apikey {
                    let apikey = apikey.clone();
                    let uploaded_course = course.clone();
                    Command::perform(Smmdb::upload_course(uploaded_course, apikey), move |res| {
                        match res {
                            Ok(res) => {
                                if !res.succeeded.is_empty() {
                                    Message::UploadSucceeded(
                                        course.clone(),
                                        res.succeeded.get(0).unwrap().id.clone(),
                                    )
                                } else {
                                    Message::ResetState
                                }
                            }
                            Err(err) => {
                                eprintln!("ERR {}", err);
                                Message::ResetState
                            }
                        }
                    })
                } else {
                    Command::none()
                }
            }
            Message::UploadSucceeded(course, id) => {
                if let Page::Save(ref mut save_page) = self.current_page {
                    save_page.set_course_response(self.smmdb.get_course_responses());
                    let index = course.get_index();
                    let mut course = course.get_course().clone();
                    course.set_smmdb_id(id.clone()).unwrap();
                    let fut = save_page.delete_course(index, self.smmdb.get_course_responses());
                    futures::executor::block_on(fut).unwrap();
                    let fut =
                        save_page.add_course(index, course, self.smmdb.get_course_responses());
                    futures::executor::block_on(fut).unwrap();
                }
                Command::batch(vec![
                    async { Message::FetchSaveCourses(vec![id]) }.into(),
                    async { Message::FetchCourses }.into(),
                    async { Message::FetchSelfCourses }.into(),
                ])
            }
            Message::InitSwapCourse(index) => {
                self.state = AppState::SwapSelect(index);
                Command::none()
            }
            Message::SwapCourse(first, second) => {
                self.state = AppState::Loading;

                match self.current_page {
                    Page::Save(ref mut save_page) => {
                        let fut = save_page.swap_courses(
                            first as u8,
                            second as u8,
                            self.smmdb.get_course_responses(),
                        );
                        futures::executor::block_on(fut).unwrap();
                        // TODO find better way than block_on
                        async { Message::ResetState }.into()
                    }
                    _ => Command::none(),
                }
            }
            Message::InitDownloadCourse(index) => {
                self.state = AppState::DownloadSelect(index);
                Command::none()
            }
            Message::DownloadCourse(save_index, smmdb_id) => {
                self.state = AppState::Downloading {
                    save_index,
                    smmdb_id,
                    progress: 0.,
                };
                Command::none()
            }
            Message::DownloadProgressed(message) => {
                if let AppState::Downloading {
                    save_index,
                    progress,
                    ..
                } = &mut self.state
                {
                    match message {
                        Progress::Started => {
                            *progress = 0.;
                        }
                        Progress::Advanced(percentage) => {
                            *progress = percentage;
                        }
                        Progress::Finished(data) => {
                            match self.current_page {
                                Page::Save(ref mut save_page) => {
                                    let course: smmdb_lib::Course2 = data.try_into().unwrap();
                                    let fut = save_page.add_course(
                                        *save_index as u8,
                                        course,
                                        self.smmdb.get_course_responses(),
                                    );
                                    futures::executor::block_on(fut).unwrap();
                                    // TODO find better way than block_on
                                    return async { Message::ResetState }.into();
                                }
                                _ => {
                                    todo!()
                                }
                            }
                        }
                        Progress::Errored => {
                            // TODO
                        }
                    }
                };
                Command::none()
            }
            Message::InitDeleteCourse(index) => {
                self.state = AppState::DeleteSelect(index);
                Command::none()
            }
            Message::DeleteCourse(index) => {
                self.state = AppState::Loading;

                match self.current_page {
                    Page::Save(ref mut save_page) => {
                        let fut =
                            save_page.delete_course(index as u8, self.smmdb.get_course_responses());
                        futures::executor::block_on(fut).unwrap();
                        // TODO find better way than block_on
                        async { Message::ResetState }.into()
                    }
                    _ => Command::none(),
                }
            }
            Message::InitDeleteSmmdbCourse(id) => {
                self.state = AppState::DeleteSmmdbSelect(id);
                Command::none()
            }
            Message::DeleteSmmdbCourse(id) => {
                self.state = AppState::Loading;

                if let Some(apikey) = &self.settings.apikey {
                    let apikey = apikey.clone();
                    Command::perform(Smmdb::delete_course(id.clone(), apikey), move |res| {
                        if let Err(err) = res {
                            eprintln!("{:?}", err);
                        }
                        Message::ReloadAfterDelete(id.clone())
                    })
                } else {
                    Command::none()
                }
            }
            Message::ReloadAfterDelete(id) => {
                self.state = AppState::Default;
                self.error_state = AppErrorState::None;

                self.smmdb.delete_course_response(id);
                if let Page::Save(ref mut save_page) = self.current_page {
                    save_page.set_course_response(self.smmdb.get_course_responses())
                }

                Command::batch(vec![
                    async { Message::FetchCourses }.into(),
                    async { Message::FetchSelfCourses }.into(),
                ])
            }
            Message::TitleChanged(title) => {
                self.smmdb.set_title(title);
                Command::none()
            }
            Message::UploaderChanged(uploader) => {
                self.smmdb.set_uploader(uploader);
                Command::none()
            }
            Message::DifficultyChanged(difficulty) => {
                self.smmdb.set_difficulty(difficulty);
                Command::none()
            }
            Message::SortChanged(sort) => {
                self.smmdb.set_sort(sort);
                Command::none()
            }
            Message::ApplyFilters => {
                self.state = AppState::Loading;
                self.smmdb.reset_pagination();
                Command::perform(
                    Smmdb::update(
                        self.smmdb.get_query_params().clone(),
                        self.settings.apikey.clone(),
                    ),
                    move |res| match res {
                        Ok(courses) => Message::SetSmmdbCourses(courses),
                        Err(err) => Message::FetchError(err.to_string()),
                    },
                )
            }
            Message::PaginateForward => {
                self.state = AppState::Loading;
                self.smmdb.paginate_forward();
                Command::perform(
                    Smmdb::update(
                        self.smmdb.get_query_params().clone(),
                        self.settings.apikey.clone(),
                    ),
                    move |res| match res {
                        Ok(courses) => Message::SetSmmdbCourses(courses),
                        Err(err) => Message::FetchError(err.to_string()),
                    },
                )
            }
            Message::PaginateBackward => {
                self.state = AppState::Loading;
                self.smmdb.paginate_backward();
                Command::perform(
                    Smmdb::update(
                        self.smmdb.get_query_params().clone(),
                        self.settings.apikey.clone(),
                    ),
                    move |res| match res {
                        Ok(courses) => Message::SetSmmdbCourses(courses),
                        Err(err) => Message::FetchError(err.to_string()),
                    },
                )
            }
            Message::PaginateSelfForward => {
                self.state = AppState::Loading;
                self.smmdb.self_paginate_forward();
                Command::perform(
                    Smmdb::update_self(
                        self.smmdb.get_own_query_params().clone(),
                        self.settings.apikey.clone(),
                    ),
                    move |res| match res {
                        Ok(courses) => Message::SetSelfSmmdbCourses(courses),
                        Err(err) => Message::FetchError(err.to_string()),
                    },
                )
            }
            Message::PaginateSelfBackward => {
                self.state = AppState::Loading;
                self.smmdb.self_paginate_backward();
                Command::perform(
                    Smmdb::update_self(
                        self.smmdb.get_own_query_params().clone(),
                        self.settings.apikey.clone(),
                    ),
                    move |res| match res {
                        Ok(courses) => Message::SetSelfSmmdbCourses(courses),
                        Err(err) => Message::FetchError(err.to_string()),
                    },
                )
            }
            Message::UpvoteCourse(course_id) => {
                if let Some(apikey) = self.settings.apikey.clone() {
                    Command::perform(
                        Smmdb::vote(course_id.clone(), 1, apikey),
                        move |res| match res {
                            Ok(()) => Message::SetVoteCourse(course_id.clone(), 1),
                            Err(err) => Message::FetchError(err),
                        },
                    )
                } else {
                    Command::none()
                }
            }
            Message::DownvoteCourse(course_id) => {
                if let Some(apikey) = self.settings.apikey.clone() {
                    Command::perform(Smmdb::vote(course_id.clone(), -1, apikey), move |res| {
                        match res {
                            Ok(()) => Message::SetVoteCourse(course_id.clone(), -1),
                            Err(err) => Message::FetchError(err),
                        }
                    })
                } else {
                    Command::none()
                }
            }
            Message::ResetCourseVote(course_id) => {
                if let Some(apikey) = self.settings.apikey.clone() {
                    Command::perform(
                        Smmdb::vote(course_id.clone(), 0, apikey),
                        move |res| match res {
                            Ok(()) => Message::SetVoteCourse(course_id.clone(), 0),
                            Err(err) => Message::FetchError(err),
                        },
                    )
                } else {
                    Command::none()
                }
            }
            Message::SetVoteCourse(course_id, value) => {
                self.smmdb.set_own_vote(course_id, value);
                if let Page::Save(ref mut save_page) = self.current_page {
                    save_page.set_course_response(self.smmdb.get_course_responses())
                }
                Command::none()
            }
            Message::OpenSettings => {
                if let Page::Settings(_) = self.current_page {
                } else {
                    self.current_page = Page::Settings(SettingsPage::new(
                        self.settings.clone(),
                        self.current_page.clone(),
                    ));
                }
                Command::none()
            }
            Message::TrySaveSettings(settings) => {
                settings.save().unwrap();
                match &settings.apikey {
                    Some(apikey) => {
                        Command::perform(Smmdb::try_sign_in(apikey.clone()), move |res| match res {
                            Ok(user) => Message::SaveSettings((settings.clone(), Some(user))),
                            Err(err) => Message::RejectSettings(err),
                        })
                    }
                    None => async move { Message::SaveSettings((settings.clone(), None)) }.into(),
                }
            }
            Message::SaveSettings((settings, user)) => {
                settings.save().unwrap();
                self.settings = settings;
                self.smmdb.set_user(user);
                if let Page::Settings(ref mut settings_page) = self.current_page {
                    self.current_page = settings_page.get_prev_page()
                }
                self.error_state = AppErrorState::None;
                Command::batch(vec![
                    async { Message::FetchCourses }.into(),
                    async { Message::FetchSelfCourses }.into(),
                ])
            }
            Message::RejectSettings(err) => {
                self.error_state = AppErrorState::Some(err);
                Command::none()
            }
            Message::CloseSettings => {
                if let Page::Settings(ref mut settings_page) = self.current_page {
                    self.current_page = settings_page.get_prev_page()
                }
                self.error_state = AppErrorState::None;
                Command::none()
            }
            Message::ChangeApiKey(apikey) => {
                if let Page::Settings(ref mut settings_page) = self.current_page {
                    settings_page.set_apikey(apikey);
                }
                Command::none()
            }
            Message::ResetApiKey => {
                if let Page::Settings(ref mut settings_page) = self.current_page {
                    settings_page.unset_apikey();
                }
                Command::none()
            }
            Message::ResetState => {
                self.state = AppState::Default;
                self.error_state = AppErrorState::None;
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        match &self.state {
            AppState::UploadSelect(_)
            | AppState::SwapSelect(_)
            | AppState::DownloadSelect(_)
            | AppState::DeleteSelect(_)
            | AppState::DeleteSmmdbSelect(_) => subscription::events().map(|event| match event {
                Event::Keyboard(keyboard::Event::KeyReleased {
                    key_code: keyboard::KeyCode::Escape,
                    modifiers: _,
                }) => Message::ResetState,
                _ => Message::Empty,
            }),
            AppState::Downloading { smmdb_id, .. } => {
                Smmdb::download_course(smmdb_id.clone()).map(Message::DownloadProgressed)
            }
            AppState::Default | AppState::Loading => Subscription::none(),
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        Container::new(
            Column::new()
                .push(
                    Row::new()
                        .push(Space::with_width(Length::Fill))
                        .push(
                            Button::new(
                                &mut self.settings_button,
                                icon::SETTINGS
                                    .clone()
                                    .width(Length::Units(24))
                                    .height(Length::Units(24)),
                            )
                            .style(DefaultButtonStyle)
                            .on_press(Message::OpenSettings),
                        )
                        .padding(12),
                )
                .push(match &mut self.current_page {
                    Page::Init(init_page) => init_page.view(&self.state, &self.error_state),
                    Page::Save(save_page) => save_page.view(&self.state, &mut self.smmdb),
                    Page::Settings(settings_page) => settings_page.view(&self.error_state),
                }),
        )
        .style(AppStyle)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

struct AppStyle;

impl container::StyleSheet for AppStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(COLOR_YELLOW)),
            ..container::Style::default()
        }
    }
}
