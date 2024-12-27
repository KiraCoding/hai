$code = "./tools/ovmf/x86_64/code.fd"
$vars = "./tools/ovmf/x86_64/vars.fd"
$src = "./target/x86_64-unknown-uefi/debug/hai_kernel.efi"
$dst = "./esp/efi/boot/bootx64.efi"

Clear-Host

# Build the kernel 
$output = cargo build --target x86_64-unknown-uefi 2>&1

# Filter warnings and errors
$warnings = ($output | Select-String -Pattern "warning:").Count
$errors = ($output | Select-String -Pattern "error:").Count

# Suppress normal output
if ($errors -eq 0) {
    Write-Output "Build completed with $warnings warning(s) and $errors error(s)."
} else {
    Write-Output "Build failed with $warnings warning(s) and $errors error(s)."
    Write-Output $output
}

if (-not (Test-Path "./esp/efi/boot")) {
    New-Item -ItemType Directory -Path "./esp/efi/boot" -Force | Out-Null
}

Copy-Item $src -Destination $dst -Force

# Run qemu
qemu-system-x86_64 `
    -drive if=pflash,format=raw,readonly=on,file=$code `
    -drive if=pflash,format=raw,readonly=on,file=$vars `
    -drive format=raw,file=fat:rw:esp