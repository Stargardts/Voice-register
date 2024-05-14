use tts::Tts;


fn main() {
    let text = "Hello, World!";

    let mut tts = Tts::default().unwrap();
    Tts::speak(&mut tts, text, false).unwrap();
}
