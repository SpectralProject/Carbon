/// ASSUME:
/// root fs is mounted and active on std::fs
/// devices that are plugged in since boot are started and mounted in /dev
/// through ACPI or device tree
/// process manager has started and can load and run ELF binaries (like init)
/// DEVICES:
/// tty0..n
/// stdin, stdout, stderr
use init::neutronapi;

// FORMAT OF /sys/config/init.yml
/*
programs:
    - arcwm
    - arcde
*/

/// INIT:
/// great resource https://www.linode.com/docs/guides/what-is-systemd/
/// set up stuff, esp other filesystems
/// setup services like __sparx_cron through neutronapi::spawn_process()
fn main() {
    // SHOULD NOT BE ASYNC spawn since order may matter
    // only allow async in the process' themselves as they spawn and join here

    let cron = neutronapi::spawn_process("/sys/bin/cron", "__sparx_cron");

    let disk_process = neutronapi::spawn_process("/sys/bin/disk", "__sparx_disk");

    let fs_process = neutronapi::spawn_process("/sys/bin/nefs_service", "__sparx_neutron_fs");

    let network_process = neutronapi::spawn_process("/sys/bin/network_service", "__sparx_network");

    let socket_process = neutronapi::spawn_process("/sys/bin/socket", "__sparx_socket");

    // GRAPHICS
    let graphics_process = neutronapi::spawn_process("/sys/bin/graphics_drm_service", "__sparx_drm");

    // spawn DE if specified
}
