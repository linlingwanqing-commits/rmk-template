#![no_std]
#![no_main]

mod keymap;
#[macro_use]
mod macros;
mod vial;

use core::ptr::addr_of_mut;

use bt_hci::controller::ExternalController;
use embassy_executor::Spawner;
use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::interrupt::software::SoftwareInterruptControl;
use esp_hal::otg_fs::Usb;
use esp_hal::otg_fs::asynch::{Config, Driver};
use esp_hal::rng::TrngSource;
use esp_hal::timer::timg::TimerGroup;
use esp_radio::ble::controller::BleConnector;
use esp_storage::FlashStorage;
use rmk::ble::{BleTransport, build_ble_stack};
use rmk::config::{BehaviorConfig, PositionalConfig, RmkConfig, StorageConfig, VialConfig};
use rmk::debounce::default_debouncer::DefaultDebouncer;
use rmk::host::HostService;
use rmk::keyboard::Keyboard;
use rmk::matrix::Matrix;
use rmk::processor::builtin::wpm::WpmProcessor;
use rmk::storage::async_flash_wrapper;
use rmk::usb::UsbTransport;
use rmk::{HostResources, KeymapData, initialize_keymap_and_storage, run_all};

use crate::keymap::*;
use crate::vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};

::esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(_s: Spawner) {
    // Initialize the peripherals and bluetooth controller
    esp_println::logger::init_logger_from_env();
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));
    esp_alloc::heap_allocator!(size: 72 * 1024);
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let software_interrupt = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, software_interrupt.software_interrupt0);
    let _trng_source = TrngSource::new(peripherals.RNG, peripherals.ADC1);
    let mut rng = esp_hal::rng::Trng::try_new().unwrap();

    let connector = BleConnector::new(peripherals.BT, Default::default()).unwrap();
    let controller: ExternalController<_, 20> = ExternalController::new(connector);
    let central_addr = [0x18, 0xe2, 0x21, 0x80, 0xc0, 0xc7];
    let mut host_resources = HostResources::new();
    let stack = build_ble_stack(controller, central_addr, &mut rng, &mut host_resources).await;

    // Initialize USB
    static mut EP_MEMORY: [u8; 1024] = [0; 1024];
    let usb = Usb::new(peripherals.USB0, peripherals.GPIO20, peripherals.GPIO19);
    // Create the driver, from the HAL.
    let config = Config::default();
    let usb_driver = Driver::new(usb, unsafe { &mut *addr_of_mut!(EP_MEMORY) }, config);

    // Initialize the flash
    let flash = FlashStorage::new(peripherals.FLASH);
    let flash = async_flash_wrapper(flash);

    // Initialize the IO pins
    let (row_pins, col_pins) = config_matrix_pins_esp!(peripherals: peripherals, input: [GPIO4, GPIO5, GPIO6, GPIO7, GPIO15], output: [GPIO16, GPIO17, GPIO18, GPIO8, GPIO9, GPIO10, GPIO11, GPIO12, GPIO13, GPIO14, GPIO21, GPIO2, GPIO1]);

    // RMK config
   // 正确的写法（使用 rmk 的语法）
// 直接写矩阵位置，如果需要宽度，使用特殊标记
let vial_config = VialConfig::new(
    VIAL_KEYBOARD_ID, 
    VIAL_KEYBOARD_DEF, 
    &[
        "0,0", "0,1", "0,2", "0,3", "0,4", "0,5", "0,6", "0,7", "0,8", "0,9", "0,10", "0,11", "0,12"
    ],
    &[
        "1,0", "1,1", "1,2", "1,3", "1,4", "1,5", "1,6", "1,7", "1,8", "1,9", "1,10", "1,11", "1,12"
    ],
    &[
        "2,0", "2,1", "2,2", "2,3", "2,4", "2,5", "2,6", "2,7", "2,8", "2,9", "2,10", "2,11", "2,12"
    ],
    &[
        "3,0", "3,1", "3,2", "3,3", "3,4", "3,5", "3,6", "3,7", "3,8", "3,9", "3,10", "3,11", "3,12"
    ],
    &[
        "4,0", "4,1", "4,2", "4,3", "4,4", "4,5",         "4,7", "4,8", "4,9", "4,10", "4,11", "4,12"
    ]
);
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

    // Initialze keyboard stuffs
    // Initialize the storage and keymap
    let mut keymap_data = KeymapData::new(keymap::get_default_keymap());
    let mut behavior_config = BehaviorConfig::default();
    let per_key_config = PositionalConfig::default();
    let (keymap, mut storage) = initialize_keymap_and_storage(
        &mut keymap_data,
        flash,
        &storage_config,
        &mut behavior_config,
        &per_key_config,
    )
    .await;

    // Initialize the matrix and keyboard
    let debouncer = DefaultDebouncer::new();
    let mut matrix = Matrix::<_, _, _, ROW, COL, true>::new(row_pins, col_pins, debouncer);
    // let mut matrix = rmk::matrix::TestMatrix::<ROW, COL>::new();
    let mut keyboard = Keyboard::new(&keymap); // Initialize the light controller
    let host_ctx = rmk::host::KeyboardContext::new(&keymap);
    let mut host_service = HostService::new(&host_ctx, &rmk_config);

    let mut usb_transport = UsbTransport::new(usb_driver, rmk_config.device_config);
    let mut ble_transport = BleTransport::new(&stack, rmk_config).await;
    let mut wpm_processor = WpmProcessor::new();

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
