#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]
use alloc::sync::Arc;
use embassy_executor::Spawner;
use esp_backtrace as _;
use esp_hal::rtc_cntl::Rtc;
use esp_println::println;
use little_weirdo::synth;
use little_weirdo::synth::data::wavetables::{BoxedWavetable, BoxedWavetables};
use little_weirdo::synth::patch::Patch;
extern crate alloc;
use esp_alloc as _;
use esp_backtrace as _;

use core::include_bytes;
use postcard;

const SAMPLE_RATE: u32 = 44_100;
const DELAY_US: u32 = 1_000_000 / SAMPLE_RATE;
#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    // init CPU
    let config = esp_hal::Config::default().with_cpu_clock(esp_hal::clock::CpuClock::_240MHz);
    let peripherals = esp_hal::init(config);
    let rtc = Rtc::new(peripherals.LPWR);
     esp_alloc::heap_allocator!(15_000);
    println!("> performance run start");
    println!("> Heap size  = {:?} bytes", esp_alloc::HEAP.free());
    println!(
        "> Heap used before allocation of wavetables/synth = {:?} bytes",
        esp_alloc::HEAP.used()
    );

    let mut wt_on_heap = BoxedWavetables::new();
    wt_on_heap.add(BoxedWavetable::new(include_bytes!("../src/soundbank/wav0.raw")));
    wt_on_heap.add(BoxedWavetable::new(include_bytes!("../src/soundbank/wav1.raw")));
    wt_on_heap.add(BoxedWavetable::new(include_bytes!("../src/soundbank/wav2.raw")));
    wt_on_heap.add(BoxedWavetable::new(include_bytes!("../src/soundbank/wav3.raw")));
    wt_on_heap.add(BoxedWavetable::new(include_bytes!("../src/soundbank/wav4.raw")));
    wt_on_heap.add(BoxedWavetable::new(include_bytes!("../src/soundbank/wav5.raw")));
    wt_on_heap.add(BoxedWavetable::new(include_bytes!("../src/soundbank/wav6.raw")));
    wt_on_heap.add(BoxedWavetable::new(include_bytes!("../src/soundbank/wav7.raw")));
    wt_on_heap.add(BoxedWavetable::new(include_bytes!("../src/soundbank/wav8.raw")));
    wt_on_heap.add(BoxedWavetable::new(include_bytes!("../src/soundbank/wav9.raw")));
    let wt = Arc::new(wt_on_heap);

    let patch_bytes: &[u8] = include_bytes!("../src/patches/ebass.lwp");
    let patch: Patch = postcard::from_bytes(patch_bytes).unwrap();
   
    let mut synth: synth::Synth = synth::Synth::new(
        SAMPLE_RATE as u16,
        &patch,
        Arc::clone(&wt),
    );
    println!(
        "> Heap used after allocation of wavetables/synth = {:?} bytes",
        esp_alloc::HEAP.used()
    );

    let mut sum = 0;
    let mut overrun = 0;
    let mut high = 0;
    let mut low: i64 = DELAY_US as i64;
    let mut moment: u32 = 0;
    for _x in 0..20 {
        let start_time = rtc.current_time().and_utc().timestamp_micros();
        synth.load_patch(&patch);
        let stop_time = rtc.current_time().and_utc().timestamp_micros();
        esp_println::println!(
            "> patch change in {} µs (max {}µs)",
            stop_time - start_time,
            DELAY_US
        );
    }

    for _x in 0..10 {
        synth.note_on(60, 100);
        synth.note_on(61, 100);

        synth.note_on(62, 100);
        synth.note_on(63, 100);
        synth.note_on(64, 100);
        synth.note_on(65, 100);

        for n in 0..SAMPLE_RATE {
            let start_time = rtc.current_time().and_utc().timestamp_micros();
            let _output = synth.clock_and_output();
            let stop_time = rtc.current_time().and_utc().timestamp_micros();
            let calculation_cost = stop_time - start_time;
            sum += calculation_cost;
            if calculation_cost < DELAY_US as i64 {
                if low > calculation_cost {
                    low = calculation_cost
                };
            } else {
                overrun += 1;
                esp_println::println!("> ! highest process time {}µs (@{})", calculation_cost, n);

                if high < calculation_cost {
                    high = calculation_cost;
                    moment = n;
                };
            }
        }

        synth.note_off(60);
        synth.note_off(61);
        synth.note_off(62);
        synth.note_off(63);
        synth.note_off(64);
        synth.note_off(65);

        for _n in 0..SAMPLE_RATE {
            synth.clock_and_output();
        }
        esp_println::println!("----- 1 sec cycle @ 44.1KHz (max 22µs ) -----");
        esp_println::println!("> average - total compute time spend  {}", sum);
        esp_println::println!("> clock :: average {}µs", sum / SAMPLE_RATE as i64);
        esp_println::println!("> deadline passed {}x,", overrun);
        esp_println::println!("> highest process time {}µs (@{})", high, moment);
        esp_println::println!("> lowest  process time {}µs", low);
        esp_println::println!("> headroom = {} %", (1_000_000 - sum) / 10000);
        esp_println::println!("----------------------------------------------");
        sum = 0;
        overrun = 0;
        high = 0;
        low = DELAY_US as i64;
    }
    println!("> performance run stop");
    
}
