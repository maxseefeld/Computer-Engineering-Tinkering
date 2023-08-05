use rust_radio::devices::{HackRFOne, Device};
use rust_radio::signal::{SignalSource, ModulatedSignal};
use rust_radio::waveforms::{SquareWave, SineWave};
use std::thread;
use std::time::Duration;

fn main() {
    let mut hackrf = HackRFOne::default().unwrap();
    hackrf.set_frequency(433.92e6).unwrap();
    hackrf.set_sample_rate(1.0e6).unwrap();

    let message = "Hello, world!";
    let symbol_rate = 1000.0;
    let carrier_frequency = 1.0e6;
    let deviation = 50_000.0;

    let square_wave = SquareWave::new(1.0 / symbol_rate);
    let sine_wave = SineWave::new(carrier_frequency);

    let modulated_signal = ModulatedSignal::new(
        &square_wave,
        &sine_wave,
        deviation,
        message.as_bytes().to_vec(),
    );

    for sample in modulated_signal.samples() {
        hackrf.send(sample).unwrap();
        thread::sleep(Duration::from_micros(1));
    }
}
