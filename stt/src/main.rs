use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound;
use pyo3::prelude::*;
use std::sync::{Arc, Mutex};

fn main() {
    // Initialize CPAL host
    let host = cpal::default_host();

    // Get the default input device
    let input_device = host
        .default_input_device()
        .expect("Failed to get default input device");

    // Get the default input format
    let input_format = input_device
        .default_input_config()
        .expect("Failed to get default input format")
        .config();

    // Create a shared buffer to store audio data
    let buffer: Arc<Mutex<Vec<i16>>> = Arc::new(Mutex::new(Vec::new()));

    // Clone the buffer to move into the input stream callback
    let buffer_clone = Arc::clone(&buffer);

    // Build the input stream
    let input_stream = input_device
        .build_input_stream(
            &input_format,
            move |data: &[i16], _: &cpal::InputCallbackInfo| {
                // Lock the buffer and write the incoming audio data to it
                let mut buffer = buffer_clone.lock().unwrap();
                buffer.extend_from_slice(data);
            },
            move |err| {
                eprintln!("Input stream error: {:?}", err);
            },
            None,
        )
        .expect("Failed to build input stream");

    // Play the input stream
    input_stream.play().expect("Failed to play input stream");

    // Keep the main thread alive to allow audio capture and playback
    println!("Recording... Press Enter to stop.");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Stop the input stream
    drop(input_stream);

    // Do something with the captured audio data
    let buffer = buffer.lock().unwrap();
    println!("Captured {} samples", buffer.len());

    // Write the audio data to a WAV file
    let spec = hound::WavSpec {
        channels: input_format.channels as u16,
        sample_rate: input_format.sample_rate.0,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer =
        hound::WavWriter::create("audio.wav", spec).expect("Failed to create WAV file");
    for sample in buffer.iter() {
        writer
            .write_sample(*sample)
            .expect("Failed to write sample");
    }
    writer.finalize().expect("Failed to finalize WAV file");

    // Initialize a python interpreter
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let sr = py
            .import_bound("speech_recognition")
            .expect("Failed to import speech_recognition");
        let recognizer = sr.getattr("Recognizer").expect("Failed to get Recognizer");

        let recognizer = recognizer.call1(()).expect("Failed to create audio file");

        let audio_handle = sr.getattr("AudioFile").expect("Failed to get AudioFile");
        let audio_file = audio_handle
            .call1(("audio.wav",))
            .expect("Failed to create audio file");
        let source = audio_file
            .call_method0("__enter__")
            .expect("Failed to enter audio file");
        recognizer.call_method1("adjust_for_ambient_noise", (&source,))
            .expect("Failed to adjust for ambient noise");
        let audio = recognizer
            .call_method1("record", (source,))
            .expect("Failed to record audio");
        let text = recognizer
            .call_method1("recognize_google", (audio, ))
            .expect("Failed to recognize audio");
        let text: String = text.extract().expect("Failed to extract text");
        println!("Recognized text: {:?}", text);
    })
}
