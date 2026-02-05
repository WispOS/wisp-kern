#!/bin/bash

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

BUILD_DIR="build"
TARGET_DIR="i386-unknown-wisp"

echo -e "${GREEN}Building wisp-kern...${NC}"

rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

echo -e "${YELLOW}Step 1: Building kernel with cargo...${NC}"
cargo build

echo -e "${YELLOW}Step 2: Converting ELF to binary...${NC}"
KERNEL_ELF="target/$TARGET_DIR/debug/kernel"
KERNEL_BIN="$BUILD_DIR/kernel.bin"
objcopy -O binary "$KERNEL_ELF" "$KERNEL_BIN"

echo -e "${YELLOW}Step 3: Assembling boot sector...${NC}"
BOOT_BIN="$BUILD_DIR/boot.bin"
nasm -f bin kernel/boot/boot.asm -o "$BOOT_BIN"

echo -e "${YELLOW}Step 4: Creating bootable image...${NC}"
OUTPUT_IMG="wisp-kern.img"

cat "$BOOT_BIN" "$KERNEL_BIN" > "$OUTPUT_IMG"

CURRENT_SIZE=$(stat -c%s "$OUTPUT_IMG")
CYLINDER_SIZE=$((63 * 512))
PADDING=$((CYLINDER_SIZE - (CURRENT_SIZE % CYLINDER_SIZE)))
if [ $PADDING -ne $CYLINDER_SIZE ]; then
    dd if=/dev/zero bs=1 count=$PADDING >> "$OUTPUT_IMG" 2>/dev/null
fi

echo -e "${GREEN}Build successful!${NC}"
echo -e "Bootable image created: ${YELLOW}$OUTPUT_IMG${NC}"
echo -e "Boot sector size: $(stat -c%s "$BOOT_BIN") bytes"
echo -e "Kernel size: $(stat -c%s "$KERNEL_BIN") bytes"
echo -e "Total image size: $(stat -c%s "$OUTPUT_IMG") bytes"

if [ "$1" == "--run" ]; then
    echo -e "${YELLOW}Launching QEMU...${NC}"
    qemu-system-i386 -drive format=raw,file="$OUTPUT_IMG",index=0,media=disk -device ati-vga
fi