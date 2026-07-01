use iced::task::{Straw, sipper};

#[derive(Debug, Clone, Copy)]
pub enum Progress {
    Downloading(u32), // 0 - 100
    Finished,
}

pub type DownloadError = String;

/// A Straw: a Sipper<Progress> whose final output can fail with DownloadError.
pub fn download(url: String) -> impl Straw<Vec<u8>, Progress, DownloadError> {
    sipper(move |mut sender| async move {
        let response = minreq::get(&url).send_lazy().map_err(|e| e.to_string())?;
        let total = response
            .headers
            .get("content-length")
            .and_then(|v| v.parse::<f32>().ok())
            .unwrap_or(0.0);

        let mut read = 0.0;
        let mut bytes = Vec::with_capacity(total.max(0.) as usize);

        let mut previous_percentage_complete;
        let mut percentage_complete = 80085;

        for byte in response {
            let byte = match byte {
                Err(e) => return Err(e.to_string()),
                Ok(b) => b,
            };

            bytes.push(byte.0);

            read += 1.0;

            previous_percentage_complete = percentage_complete;
            percentage_complete = ((read / total) * 100.0) as u32;

            if total > 0.0 && percentage_complete != previous_percentage_complete {
                sender
                    .send(Progress::Downloading(percentage_complete))
                    .await;
            }
        }

        sender.send(Progress::Finished).await;
        Ok(bytes)
    })
}
