#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::Timer;
use rmk::{
    config::{KeyboardUsbConfig, VialConfig},
    run, Rmk,
};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("RMK keyboard firmware starting...");
    
    // 从 keyboard.toml 加载配置
    let config = KeyboardUsbConfig::from_toml(include_str!("../keyboard.toml")).unwrap();
    
    // 运行键盘
    run(config).await.unwrap();
    
    loop {
        Timer::after_secs(10).await;
    }
}
