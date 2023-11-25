/// Downloads a video from YouTube using the provided URL.
///
/// Requires the `pytube` Python library installed in the Python environment.
/// 
/// # Arguments
///
/// * `youtube_url` - The URL of the YouTube video to download.
///
/// # Returns
///
/// A `PyResult<()>` indicating success or failure.



use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

pub async fn download_youtube(youtube_url: &str) -> PyResult<()> {
    let youtube_url = Box::leak(youtube_url.to_string().into_boxed_str());
    let download_handle = tokio::task::spawn_blocking(move || {
        Python::with_gil(|py| {
            let locals = [("pytube", py.import("pytube")?)].into_py_dict(py);
            let py_code = format!(
                "pytube.YouTube('{}').streams.filter(file_extension='mp4', res='1080p', adaptive=True).first().download()",
                youtube_url
            );

            py.run(&py_code, None, Some(locals))?;

            Ok::<(), PyErr>(())
        })
    });

    let _ = download_handle.await; // Properly propagate errors

    Ok(())
}
