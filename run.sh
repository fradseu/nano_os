#!/usr/bin/env bash
set -e

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT_DIR"

cargo build --manifest-path kernel/Cargo.toml

rm -rf iso_root
mkdir -p iso_root/boot
cp kernel/target/x86_64-unknown-none/debug/kernel iso_root/boot/kernel
cp limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin limine.conf iso_root/boot/

xorriso -as mkisofs -b boot/limine-bios-cd.bin \
    -no-emul-boot -boot-load-size 4 -boot-info-table \
    --efi-boot boot/limine-uefi-cd.bin \
    -efi-boot-part --efi-boot-image --protective-msdos-label \
    iso_root -o nano_os.iso

./limine/limine bios-install nano_os.iso

qemu-system-x86_64 -cdrom nano_os.iso -serial stdio -display none
