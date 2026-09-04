{
  modulesPath,
  lib,
  pkgs,
  ...
} @ args:
{
  imports = [
    (modulesPath + "/installer/scan/not-detected.nix")
    (modulesPath + "/profiles/qemu-guest.nix")
    ./disk-config.nix
    ./postgres.nix
  ];

  networking = {
    hostName = "big-sky";
    # networkmanager.enable = true;
    # firewall.allowedTCPPorts = [ 3000 ];
  };

  boot.loader.grub = {
    efiSupport = true;
    efiInstallAsRemovable = true;
  };

  services = {

    openssh.enable = true;

    # INFO: Only enabled once we're up and running
    openssh.settings = {
      PasswordAuthentication = false;
      KbdInteractiveAuthentication = false;
      PermitRootLogin = "prohibit-password";
    };

  };

  nixpkgs.config.allowUnfree = true;
  environment.systemPackages = map lib.lowPrio [
    pkgs.curl
    pkgs.gitMinimal
    pkgs.neovim
    pkgs.gh
    pkgs.ghostty.terminfo
  ];

  # INFO: temp password needed initially
  # users.users.root.initialPassword = "1234";

  # Only allow connections from confirmed users
  # These are Public Keys and can be commited to version control.
  users.users.root.openssh.authorizedKeys.keys =
  [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIFQqcoRpf/3hi+RZtrsx3V/mgp8xOQ+Q+tNfE3PLXEyl"
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIEYi1z8oaqJLZyUd+jh1TI323ThXJPljm7hGgDBmNFp+"
  ] ++ (args.extraPublicKeys or []); # this is used for unit-testing this module and can be removed if not needed

  # WARNING: SERIOUSLY... DO NOT TOUCH
  system.stateVersion = "24.05";
}
