use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyleBuilder,
};
use linux_embedded_hal::I2cdev;
use ssd1306::{displaysize::DisplaySize128x32, mode::GraphicsMode, Builder, I2CDIBuilder};
extern crate ctrlc;
use serde_json::Value;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time};

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let i2c = I2cdev::new("/dev/i2c-1").unwrap();

    let interface = I2CDIBuilder::new().init(i2c);
    let mut disp: GraphicsMode<_, _> = Builder::new()
        .size(DisplaySize128x32)
        .connect(interface)
        .into();

    disp.init().unwrap();
    disp.flush().unwrap();

    let text_style = TextStyleBuilder::new(Font6x8)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .build();

    let one_sec = time::Duration::from_secs(1);

    while running.load(Ordering::SeqCst) {
        let body = reqwest::blocking::get("http://localhost:26657/status")
            .expect("URL Failed")
            .text()
            .unwrap();
        let res: Value = serde_json::from_str(body.as_str()).unwrap();
        if let height = res["result"]["sync_info"]["latest_block_height"]
            .as_str()
            .unwrap()
        {
            Text::new(height, Point::zero())
                .into_styled(text_style)
                .draw(&mut disp)
                .unwrap();
            disp.flush().unwrap();

            thread::sleep(one_sec);
        }
    }

    disp.clear();
    disp.flush().unwrap();
}
