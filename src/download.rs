pub struct Download {
    pub url: String,
}

impl<H, I> iced_native::subscription::Recipe<H, I> for Download
where
    H: std::hash::Hasher,
{
    type Output = Progress;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;

        std::any::TypeId::of::<Self>().hash(state);
        self.url.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        Box::pin(futures::stream::unfold(
            State::Ready(self.url),
            |state| async move {
                match state {
                    State::Ready(url) => {
                        let response = reqwest::get(&url).await;

                        match response {
                            Ok(response) => {
                                if let Some(total) = response.content_length() {
                                    Some((
                                        Progress::Started,
                                        State::Downloading {
                                            response,
                                            total,
                                            downloaded: 0,
                                            data: vec![],
                                        },
                                    ))
                                } else {
                                    Some((Progress::Errored, State::Finished))
                                }
                            }
                            Err(_) => Some((Progress::Errored, State::Finished)),
                        }
                    }
                    State::Downloading {
                        mut response,
                        total,
                        downloaded,
                        mut data,
                    } => match response.chunk().await {
                        Ok(Some(chunk)) => {
                            let downloaded = downloaded + chunk.len() as u64;
                            data.extend(chunk.iter().cloned());

                            let percentage = (downloaded as f32 / total as f32) * 100.0;

                            Some((
                                Progress::Advanced(percentage),
                                State::Downloading {
                                    response,
                                    total,
                                    downloaded,
                                    data,
                                },
                            ))
                        }
                        Ok(None) => Some((Progress::Finished(data), State::Finished)),
                        Err(_) => Some((Progress::Errored, State::Finished)),
                    },
                    State::Finished => {
                        // We do not let the stream die, as it would start a
                        // new download repeatedly if the user is not careful
                        // in case of errors.
                        let _: () = iced::futures::future::pending().await;

                        None
                    }
                }
            },
        ))
    }
}

#[derive(Debug, Clone)]
pub enum Progress {
    Started,
    Advanced(f32),
    Finished(Vec<u8>),
    Errored,
}

pub enum State {
    Ready(String),
    Downloading {
        response: reqwest::Response,
        total: u64,
        downloaded: u64,
        data: Vec<u8>,
    },
    Finished,
}
