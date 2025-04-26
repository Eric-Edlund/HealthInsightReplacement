
Build a nix vm image using 

https://nix.dev/tutorials/nixos/nixos-configuration-on-vm.html

Take the nixpkgs/nixos base object, use the vm attribute which they maintain,
provide the local config to the evaluation path.

```bash
nix-build '<nixpkgs/nixos>' -A vm  -I nixos-config=./configuration.nix
```

This creates a link file `result` in cwd to the built machine.
Run it with:

```bash
# Run in the current terminal -nographic
# Show the boot process console=ttyS0
QEMU_KERNEL_PARAMS=console=ttyS0 ./result/bin/run-nixos-vm -nographic; reset
```

Nix can even write an iso image:

https://nix.dev/tutorials/nixos/building-bootable-iso-image#bootable-iso-image
