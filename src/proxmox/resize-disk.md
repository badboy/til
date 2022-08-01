# Resize disks of a VM

Source: <https://pve.proxmox.com/wiki/Resize_disks>

1. Extend the disk in the web UI
2. Run `partprobe` on the machine
3. Run `parted`
  * `print` to show current layout. This will ask you to fix the GPT. Say "Fix"
  * `resizepart 1` -- `1` being the partition ID
  * It asks for the end. Type `100%`
4. Resize the filesystem: `resize2fs /dev/vda1`
