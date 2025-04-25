use clap::Parser;
use clap_derive::Parser;
use std::io::{BufRead, Read, Seek};
use std::path::Path;
use std::{path::PathBuf, sync::Arc};

use crate::vmware::{vmrun::VmState, VMWare};

pub mod vmware;

fn install_bootx64(root: &Path, source: &Path) -> anything::Result<()> {
    // check if the path exists
    if !source.exists() {
        return Err(format!("Source {} does not exist", source.to_string_lossy()).into());
    }

    // create /efi/boot if it doesn't exist
    let dest = root.join("efi/boot");
    if !dest.exists() {
        std::fs::create_dir_all(&dest)?;
        log::debug!("Created directory {}", dest.to_string_lossy());
    }

    // copy to dest
    std::fs::copy(source, dest.join("bootx64.efi"))?;
    log::debug!(
        "Copied bootx64.efi from {} to {}",
        source.to_string_lossy(),
        dest.to_string_lossy()
    );

    Ok(())
}

fn serial_loop(vm: &VMWare, project: &Path) -> anything::Result<()> {
    // setup exit handler
    let shared_vm = Arc::new(vm.clone());

    ctrlc::set_handler(move || {
        log::debug!("Received termination, stopping VM...");
        if shared_vm.stop(false).is_ok() {
            log::info!("VM Terminated");
            return;
        }

        // force terminate
        log::warn!("Failed to stop VM, forcing termination...");
        if shared_vm.stop(true).is_ok() {
            log::info!("VM Terminated");
            return;
        }

        log::error!("Failed to terminate VM");
    })?;

    let mut cursor = 0;
    let log_path = project.join("vmware.log");
    let mut file = std::fs::OpenOptions::new().read(true).open(&log_path)?;

    // read serial output

    let mut leave_time = None;
    loop {
        match vm.state() {
            Ok(VmState::Running) => {
                leave_time = None;
            }
            _ => {
                // Set `leave_time` to 2 seconds from now if not already set
                leave_time.get_or_insert_with(|| {
                    std::time::Instant::now() + std::time::Duration::from_secs(2)
                });
            }
        }

        if let Some(t) = leave_time {
            if std::time::Instant::now() >= t {
                break;
            }
        }

        file.seek(std::io::SeekFrom::Start(cursor))?;

        let mut reader = std::io::BufReader::new(&file);

        for line in reader.by_ref().lines() {
            let line = line?;

            let blacklist = [
                "mksSandboxLog",
                "USBGW",
                "USBArbLib",
                "PowerOn",
                "SSLConfigLoad",
                "Bluetooth",
                "GuestRpc",
                "In(05)",
                "No(00)",
                "Er(02)",
                "hostVerifiedSamlToken",
                "HDAudio",
                "Balloon",
                "mks",
                "VMCI",
                "vmx Guest",
            ];

            if !blacklist.iter().any(|x| line.contains(x)) {
                println!("{}", line.replace(" Wa(03)", ""));
            }
        }

        cursor = reader.get_mut().seek(std::io::SeekFrom::End(0))?;
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    log::info!("Exited serial loop");
    Ok(())
}

fn wait_vm_termination(vm: &VMWare) {
    // wait for vm to terminate
    while let Ok(VmState::Running) = vm.state() {
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}

struct RunParams {
    bootx64: String,
    project: String,
    root: String,
    password: Option<String>,
}

fn find_vmx(directory: &PathBuf) -> anything::Result<PathBuf> {
    let mut vmx = None;
    for entry in std::fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map(|x| x == "vmx").unwrap_or(false) {
            vmx = Some(path);
            break;
        }
    }

    vmx.ok_or_else(|| format!("No vmx file found in {}", directory.to_string_lossy()).into())
}

fn run(params: RunParams) -> anything::Result<()> {
    let project = dunce::canonicalize(&params.project)?;
    let vmx = find_vmx(&project)?;
    log::debug!("Found VMX file: {}", vmx.to_string_lossy());

    let vm = VMWare::new(&vmx, params.password)?;

    log::debug!("Checking VM...");
    if vm.state()? == VmState::Running {
        log::debug!("VM is already running, stopping...");
        vm.stop(true)?;
        log::info!("Terminated already running VM");
    }

    log::debug!("Installing UEFI...");
    let root = dunce::canonicalize(&params.root)?;
    let bootx64 = dunce::canonicalize(&params.bootx64)?;
    install_bootx64(&root, &bootx64)?;
    log::info!("Installed UEFI to {}", root.to_string_lossy());

    log::debug!("Launching VM from {}", project.to_string_lossy());
    vm.start(true)?;
    log::info!("Launched VM");

    log::debug!("Entering message loop...");
    serial_loop(&vm, &project)?;

    log::debug!("Waiting for VM termination...");
    wait_vm_termination(&vm);
    log::info!("VM Terminated");

    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    vmproj: String,

    #[arg(short = 'p', long = "password")]
    password: Option<String>,
    device: String,
    efi: String,
}

fn main() -> anything::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let args = Args::parse();

    let params = RunParams {
        bootx64: args.efi,
        project: args.vmproj,
        root: args.device,
        password: args.password,
    };

    match run(params) {
        Ok(_) => log::info!("Finished"),
        Err(e) => log::error!("{}", e),
    }

    Ok(())
}
