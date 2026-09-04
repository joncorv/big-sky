# Nixos configured for remote deployment

To install using nixos-anywhere on any computer with nix installed, and flakes enabled:
replace user@ip-address with your server credentials

```bash
nix run github:nix-community/nixos-anywhere -- \
  --kexec-extra-flags "--kexec-syscall" \
  --flake .#generic --generate-hardware-config nixos-generate-config ./hardware-configuration.nix \
  user@ip-address
```

Then to remote in and rebuild / update the system
replace user@ip-address with your server credentials

```bash
nix run nixpkgs#nixos-rebuild -- \
  switch \
  --no-reexec \
  --flake .#generic \
  --target-host user@ip-address \
  --build-host user@ip-address
```
