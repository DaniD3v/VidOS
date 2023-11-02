use std::error::Error;
use ffmpeg::format::{input, Pixel};
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::util::frame::video::Video;
use std::path::Path;
use image::RgbImage;

pub fn for_each_frame(path: &Path, run: &mut dyn FnMut(RgbImage) -> Result<(), Box<dyn Error>>) -> Result<(), Box<dyn Error>> {
    let mut ictx = input(&path)?;

    let input = ictx
        .streams()
        .best(Type::Video)
        .ok_or(ffmpeg::Error::StreamNotFound)?;
    let video_stream_index = input.index();

    let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
    let mut decoder = context_decoder.decoder().video()?;

    let mut scaler = Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        Flags::BILINEAR,
    )?;

    let mut receive_and_process_decoded_frames =
        |decoder: &mut ffmpeg::decoder::Video| -> Result<(), Box<dyn Error>> {
            let mut decoded = Video::empty();
            while decoder.receive_frame(&mut decoded).is_ok() {
                let mut video_frame = Video::empty();
                scaler.run(&decoded, &mut video_frame)?;

                let raw = video_frame.data(0).to_vec();
                let frame = image::RgbImage::from_raw(video_frame.width(), video_frame.height(), raw);
                let frame = frame.unwrap();

                run(frame)?
            }
            Ok(())
        };

    for (stream, packet) in ictx.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet)?;
            receive_and_process_decoded_frames(&mut decoder)?;
        }
    }
    decoder.send_eof()?;
    receive_and_process_decoded_frames(&mut decoder)?;

    Ok(())
}