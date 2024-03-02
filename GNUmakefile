
override BOOTLOADER_UEFI_BIN := bootloader-uefi/target/x86_64-unknown-uefi/debug

.PHONY: all clean qemu

all:
	$(MAKE) ovmf

	cd bootloader-uefi && cargo build

	rm -f uefi-cd.img
	dd if=/dev/zero of=uefi-cd.img bs=512 count=2880
	mformat -i uefi-cd.img -f 1440 -N 12345678 ::
	mmd -i  uefi-cd.img ::/EFI ::/EFI/BOOT
	mcopy -i uefi-cd.img $(BOOTLOADER_UEFI_BIN)/bootloader-uefi.efi ::/EFI/BOOT/BOOTX64.EFI

	rm -rf iso_root
	mkdir -p iso_root
	mkdir -p iso_root/boot
	cp uefi-cd.img iso_root/boot
	xorriso -as mkisofs \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot boot/uefi-cd.img \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o OROS.iso
	
	rm -f uefi-cd.img
	rm -rf iso_root


ovmf:
	mkdir -p ovmf
	cd ovmf && curl -Lo OVMF.fd https://retrage.github.io/edk2-nightly/bin/RELEASEX64_OVMF.fd

qemu: all ovmf
	qemu-system-x86_64 -enable-kvm -cpu host -m 2G -bios ovmf/OVMF.fd -cdrom OROS.iso -boot d

clean:
	cd bootloader-uefi && cargo clean
	rm -f OROS.iso