use tts::Tts;
// use std::
    // {thread,
    // time::Duration}
// ;


fn main() {
    let text = "Hello, World!";

    let mut speaker = Tts::default().unwrap();

    let voices = Tts::voices(&speaker).unwrap();

    for voice in voices {
        // Print English Voices
        if voice.language() == "en-US" {
            Tts::set_voice(&mut speaker, &voice).unwrap();
            Tts::speak(&mut speaker, text, false).unwrap();
            println!("Voice: {}", voice.name());
            // thread::sleep(Duration::from_secs(1));
        }
    }
}
