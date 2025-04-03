use std::{error::Error, fs::File, sync::Mutex};

use crash_handler::{CrashContext, CrashEventResult, CrashHandler, make_crash_event};
use pelite::pe64::PeObject;
use std::ptr;
use tracing_panic::panic_hook;
use util::{program::Program, system::wait_for_system_init};
pub const DLL_PROCESS_ATTACH: u32 = 1;


/// # DllMain Entry Point
///
/// This is the entry point for the DLL. It is called by the operating system when the DLL is loaded or unloaded.
///
/// ## Parameters
/// - `_hmodule`: A handle to the DLL module. This parameter is unused in this implementation.
/// - `reason`: The reason code for the call. This implementation only handles `DLL_PROCESS_ATTACH` (value `1`).
///
/// ## Behavior
/// When the `reason` is `DLL_PROCESS_ATTACH`, the following actions are performed:
/// 1. The `setup` function is called to initialize logging, crash handling, and other setup tasks.
/// 2. A new thread is spawned to:
///    - Wait for the system to initialize using `wait_for_system_init`.
///    - Call the `init` function to perform the main DLL initialization logic.
///
/// ## Safety
/// This function is marked as `unsafe` because it interacts with low-level system APIs and performs operations
/// that require careful handling, such as spawning threads and modifying memory.
///
/// ## Returns
/// Always returns `true` to indicate successful execution.
/// ```
#[unsafe(no_mangle)]
#[allow(unsafe_code)]
pub unsafe extern "C" fn DllMain(_hmodule: usize, reason: u32) -> bool {
    if reason == DLL_PROCESS_ATTACH {
        setup().unwrap();

        std::thread::spawn(|| {
            // Give the CRT init a bit of leeway
            wait_for_system_init(5000).expect("System initialization timed out");

            init().expect("Could not start the DLL after the game would be ready");
        });
    }
    true
}

fn setup() -> Result<(), Box<dyn Error>> {
    let log_file = File::create("./er_skip_splash_screens.log")?;
    let subscriber = tracing_subscriber::fmt()
        .with_writer(Mutex::new(log_file))
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    std::panic::set_hook(Box::new(panic_hook));

    let handler = CrashHandler::attach(unsafe {
        make_crash_event(move |context: &CrashContext| {
            tracing::error!(
                "Exception: {:x} at {:x}",
                context.exception_code,
                (*(*context.exception_pointers).ExceptionRecord).ExceptionAddress as usize
            );

            CrashEventResult::Handled(true)
        })
    })
    .unwrap();
    std::mem::forget(handler);
    Ok(())
}

fn init() -> Result<(), Box<dyn Error>> {
    let program: Program<'_> = unsafe { Program::current() };
    let pe_image: &[u8] = program.image();

    let start_of_jump_instruction = 0xB0C3ED;
    let offsets: [isize; 1] = [start_of_jump_instruction];
    let full_replacement = [0x7F]; // JG
    let expected_existing_bytes = [0x74]; // JZ

    valid_replacement(pe_image, offsets, expected_existing_bytes).unwrap();

    let target_address = unsafe { pe_image.as_ptr().offset(offsets[0]) as *mut u8 };

    unsafe {
        for (i, &byte) in full_replacement.iter().enumerate() {
            ptr::write(target_address.add(i), byte);
        }
    }
    tracing::info!(
        "ER Skip Startup Cutscenes injected successfully at: eldenring.exe:{:x?}",
        offsets,
    );
    tracing::info!(
        "ER Skip Startup Cutscenes replaced {:x?} with: {:x?}",
        expected_existing_bytes,
        full_replacement
    );
    Ok(())
}

fn valid_replacement(pe_image: &[u8], offsets: [isize; 1], expected_existing_bytes: [u8; 1]) -> Result<(), Box<dyn Error>> {
    for (i, &offset) in offsets.iter().enumerate() {
        // Calculate the target memory address
        let target_address = unsafe { pe_image.as_ptr().offset(offset) as *mut u8 };

        // Check if the target address is valid
        assert!(!target_address.is_null(), "Target address is null at offset: {:x}", offset);

        // Check that the target address has the expected opcode
        let val = unsafe { *target_address };
        assert_eq!(val, expected_existing_bytes[i], "Unexpected opcode at target address (offset {:x}): {:x}", offset, val);
    }
    Ok(())
}
