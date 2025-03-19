use ovmf_prebuilt::{Arch, FileType, Prebuilt, Source};

fn main() {
    // read env variables that were set in build script
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");

    // choose whether to start the UEFI or BIOS image
    let uefi = true;
    let prebuilt =
        Prebuilt::fetch(Source::LATEST, "target/ovmf").expect("failed to update prebuilt");
    let bios_code = prebuilt.get_file(Arch::X64, FileType::Code);
    let bios_vars = prebuilt.get_file(Arch::X64, FileType::Vars);

    let bios_path_code = bios_code.to_str().unwrap();
    let bios_path_vars = bios_vars.to_str().unwrap();

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    if uefi {
        // deprecated
        // cmd.arg("-bios")
        //     .arg(prebuilt.get_file(Arch::X64, FileType::Code));
        cmd.arg("-drive")
            .arg(format!("if=pflash,format=raw,file={bios_path_code}"));
        cmd.arg("-drive")
            .arg(format!("if=pflash,format=raw,file={bios_path_vars}"));
        cmd.arg("-drive")
            .arg(format!("format=raw,file={uefi_path}"));
    } else {
        cmd.arg("-drive")
            .arg(format!("format=raw,file={bios_path}"));
    }
    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}
