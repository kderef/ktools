use iced::task::{Straw, sipper};

#[derive(Debug, Clone, Copy)]
pub enum Progress {
    Downloading(f32), // 0.0 - 100.0
    Finished,
}

pub type DownloadError = String;

/// A Straw: a Sipper<Progress> whose final output can fail with DownloadError.
pub fn download(url: String) -> impl Straw<(), Progress, DownloadError> {
    sipper(move |mut sender| async move {
        let response = minreq::get(&url).send_lazy().map_err(|e| e.to_string())?;

        let total = response.size_hint().0 as f32;
        let mut read: f32 = 0.0;

        for byte in response {
            byte.map_err(|e| e.to_string())?;

            read += 1.0;
            if total > 0.0 {
                sender
                    .send(Progress::Downloading((read / total) * 100.0))
                    .await;
            }
        }

        sender.send(Progress::Finished).await;
        Ok(())
    })
}
