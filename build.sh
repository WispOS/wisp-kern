#!/bin/bash
set -e

KARCH="x86_64"
DISK_DIR="disks"
IMAGE_NAME="wisp-${KARCH}"
QEMUFLAGS="-m 2G"
ISO_ROOT="iso_root"
LIMINE_DIR="limine"

build_kernel() {
    make -C kernel
}

setup_limine() {
    if [ ! -d "${LIMINE_DIR}" ]; then
        git clone https://github.com/limine-bootloader/limine.git --branch=v8.x-binary --depth=1
    fi
}

build_iso() {
    setup_limine
    rm -rf "${ISO_ROOT}"
    mkdir -p "${ISO_ROOT}/boot" "${ISO_ROOT}/EFI/BOOT"
    
    if [ ! -f "kernel/kernel" ]; then exit 1; fi
    cp kernel/kernel "${ISO_ROOT}/boot/"
    
    cp "${LIMINE_DIR}/limine-bios.sys" "${ISO_ROOT}/boot/" 2>/dev/null || true
    cp "${LIMINE_DIR}/limine-bios-cd.bin" "${ISO_ROOT}/boot/"
    cp "${LIMINE_DIR}/limine-uefi-cd.bin" "${ISO_ROOT}/boot/" 2>/dev/null || true
    cp "${LIMINE_DIR}/BOOTX64.EFI" "${ISO_ROOT}/EFI/BOOT/" 2>/dev/null || true
    cp "${LIMINE_DIR}/BOOTIA32.EFI" "${ISO_ROOT}/EFI/BOOT/" 2>/dev/null || true
    
    cat > "${ISO_ROOT}/boot/limine.conf" << 'EOF'
timeout: 0
/wisp-kern
    protocol: limine
    kernel_path: boot():/boot/kernel
EOF
    
    xorriso -as mkisofs \
        -b boot/limine-bios-cd.bin \
        -no-emul-boot -boot-load-size 4 -boot-info-table \
        --efi-boot boot/limine-uefi-cd.bin \
        -efi-boot-part --efi-boot-image --protective-msdos-label \
        "${ISO_ROOT}" -o "${IMAGE_NAME}.iso"
    
    if [ -f "${LIMINE_DIR}/limine" ]; then
        "${LIMINE_DIR}/limine" bios-install "${IMAGE_NAME}.iso"
    fi
}

create_disks() {
    mkdir -p "${DISK_DIR}"
    [ -f "${DISK_DIR}/ide_disk.img" ] || dd if=/dev/zero of="${DISK_DIR}/ide_disk.img" bs=1M count=64 status=none
    [ -f "${DISK_DIR}/ahci_disk.img" ] || dd if=/dev/zero of="${DISK_DIR}/ahci_disk.img" bs=1M count=64 status=none
}

find_ovmf() {
    SYS_CODE=$(find /usr/share/edk2* -name "OVMF_CODE*.fd" | head -n 1)
    SYS_VARS=$(find /usr/share/edk2* -name "OVMF_VARS*.fd" | head -n 1)

    
    if [[ -z "$SYS_CODE" ]]; then 
        SYS_CODE=$(find /usr/share/OVMF* -name "OVMF_CODE*.fd" | head -n 1)
        SYS_VARS=$(find /usr/share/OVMF* -name "OVMF_VARS*.fd" | head -n 1)
    fi

    mkdir -p ovmf
    cp "$SYS_CODE" ovmf/ovmf-code.fd
    cp "$SYS_VARS" ovmf/ovmf-vars.fd
    chmod 644 ovmf/ovmf-vars.fd
    
    OVMF_CODE="ovmf/ovmf-code.fd"
    OVMF_VARS="ovmf/ovmf-vars.fd"
}


run_qemu() {
    find_ovmf
    qemu-system-${KARCH} \
        -M pc \
        -drive if=pflash,unit=0,format=raw,file="$OVMF_CODE",readonly=on \
        -drive if=pflash,unit=1,format=raw,file="$OVMF_VARS" \
        -cdrom "${IMAGE_NAME}.iso" \
        -drive file="${DISK_DIR}/ide_disk.img",format=raw,if=ide,index=0,media=disk \
        -drive file="${DISK_DIR}/ahci_disk.img",format=raw,if=none,id=ahci0 \
        -device ahci,id=ahci \
        -device ide-hd,drive=ahci0,bus=ahci.0 \
        -smp 4 ${QEMUFLAGS}
}

run_qemu_codespace() {
    find_ovmf
    qemu-system-${KARCH} \
        -M pc -display curses \
        -drive if=pflash,unit=0,format=raw,file="$OVMF_CODE",readonly=on \
        -drive if=pflash,unit=1,format=raw,file="$OVMF_VARS" \
        -cdrom "${IMAGE_NAME}.iso" \
        ${QEMUFLAGS}
}

case "${1:-}" in
    build) build_kernel; build_iso ;;
    run) build_kernel; build_iso; create_disks; run_qemu ;;
    run-codespace) build_kernel; build_iso; create_disks; run_qemu_codespace ;;
    clean) make -C kernel clean; rm -rf "${ISO_ROOT}" "${IMAGE_NAME}.iso" ;;
    distclean) make -C kernel clean; rm -rf "${ISO_ROOT}" "${IMAGE_NAME}.iso" "${LIMINE_DIR}" "${DISK_DIR}" ;;
    *) echo "Usage: $0 {build|run|run-codespace|clean|distclean}"; exit 1 ;;
esac
