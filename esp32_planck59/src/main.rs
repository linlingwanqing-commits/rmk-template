#![no_std]
#![no_main]

mod keymap;
mod macros;
mod vial;

use embassy_executor::Spawner;
use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::gpio::{Input, Output, Pull};
use esp_hal::rng::Trng;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::Config;
use esp_radio::ble::controller::BleConnector;
use esp_storage::FlashStorage;
use rmk::ble::{build_ble_stack, BleTransport};
use rmk::config::{RmkConfig, StorageConfig, VialConfig};
use rmk::debounce::default_debouncer::DefaultDebouncer;
use rmk::matrix::Matrix;
use rmk::storage::async_flash_wrapper;
use rmk::usb::UsbTransport;
use rmk::{initialize_keymap_and_storage, run_all, KeymapData};

use crate::keymap::*;
use crate::vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};

#[esp_rtos::main]
async fn main(_s: Spawner) {
    esp_println::logger::init_logger_from_env();

    // 初始化硬件
    let peripherals = esp_hal::init(Config::default());
    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let mut rng = Trng::new(peripherals.RNG, peripherals.ADC1);

    // 初始化 BLE
    let connector = BleConnector::new(peripherals.BT, Default::default()).unwrap();
    let central_addr = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC];
    let mut host_resources = rmk::HostResources::new();
    let stack = build_ble_stack(connector, central_addr, &mut rng, &mut host_resources).await;

    // 初始化存储
    let flash = FlashStorage::new(peripherals.FLASH);
    let flash = async_flash_wrapper(flash);

    // 初始化矩阵引脚 (5行 x 13列)
    let row_pins = [
        Input::new(peripherals.GPIO4, Pull::Up),
        Input::new(peripherals.GPIO5, Pull::Up),
        Input::new(peripherals.GPIO6, Pull::Up),
        Input::new(peripherals.GPIO7, Pull::Up),
        Input::new(peripherals.GPIO15, Pull::Up),
    ];

    let col_pins = [
        Output::new(peripherals.GPIO16, esp_hal::Level::High),
        Output::new(peripherals.GPIO17, esp_hal::Level::High),
        Output::new(peripherals.GPIO18, esp_hal::Level::High),
        Output::new(peripherals.GPIO8, esp_hal::Level::High),
        Output::new(peripherals.GPIO9, esp_hal::Level::High),
        Output::new(peripherals.GPIO10, esp_hal::Level::High),
        Output::new(peripherals.GPIO11, esp_hal::Level::High),
        Output::new(peripherals.GPIO12, esp_hal::Level::High),
        Output::new(peripherals.GPIO13, esp_hal::Level::High),
        Output::new(peripherals.GPIO14, esp_hal::Level::High),
        Output::new(peripherals.GPIO21, esp_hal::Level::High),
        Output::new(peripherals.GPIO2, esp_hal::Level::High),
        Output::new(peripherals.GPIO1, esp_hal::Level::High),
    ];

    // Vial 配置
    let vial_config = VialConfig::new(VIAL_KEYBOARD_ID, VIAL_KEYBOARD_DEF, &[]);

    let storage_config = StorageConfig {
        start_addr: 0x3f0000,
        num_sectors: 16,
        ..Default::default()
    };

    let rmk_config = RmkConfig {
        vial_config,
        storage_config,
        ..Default::default()
    };

    // 初始化键盘
    let mut keymap_data = KeymapData::new(get_default_keymap());
    let mut behavior_config = Default::default();
    let per_key_config = Default::default();
    let (keymap, mut storage) = initialize_keymap_and_storage(
        &mut keymap_data,
        flash,
        &storage_config,
        &mut behavior_config,
        &per_key_config,
    )
    .await;

    let debouncer = DefaultDebouncer::new();
    let mut matrix = Matrix::new(row_pins, col_pins, debouncer);
    let mut keyboard = rmk::keyboard::Keyboard::new(&keymap);
    let host_ctx = rmk::host::KeyboardContext::new(&keymap);
    let mut host_service = rmk::host::HostService::new(&host_ctx, &rmk_config);

    // USB 配置
    let usb = esp_hal::otg_fs::Usb::new(peripherals.USB0, peripherals.GPIO20, peripherals.GPIO19);
    static mut EP_MEMORY: [u8; 1024] = [0; 1024];
    let usb_driver = esp_hal::otg_fs::asynch::Driver::new(usb, unsafe { &mut EP_MEMORY }, Default::default());
    let usb_transport = UsbTransport::new(usb_driver, rmk_config.device_config);

    let ble_transport = BleTransport::new(&stack, rmk_config).await;
    let wpm_processor = rmk::processor::builtin::wpm::WpmProcessor::new();

    run_all!(
        matrix,
        storage,
        usb_transport,
        ble_transport,
        wpm_processor,
        keyboard,
        host_service
    )
    .await;
}