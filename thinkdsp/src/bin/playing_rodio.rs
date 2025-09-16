use std::{error::Error, io::BufReader, thread, time::Duration};

use rodio::{Decoder, OutputStreamBuilder};

fn main() -> Result<(), Box<dyn Error>> {
    let stream_handle = OutputStreamBuilder::open_default_stream()?;

    // rodio::play
    let file = std::fs::File::open("thinkdsp/presets/268050__sceza__bass-sine-sweep-10-50hz.wav")?;
    let sink = rodio::play(&stream_handle.mixer(), BufReader::new(file))?;
    sink.set_volume(0.5);
    sink.sleep_until_end();

    println!("1Playing wav file");
    thread::sleep(Duration::from_millis(3500));
    sink.set_volume(0.0);

    // mixer.add
    let file = std::fs::File::open("thinkdsp/presets/268050__sceza__bass-sine-sweep-10-50hz.wav")?;
    let source = Decoder::try_from(file).unwrap();
    stream_handle.mixer().add(source);
    println!("2Playing wav file");
    thread::sleep(Duration::from_millis(3500));

    Ok(())
}
