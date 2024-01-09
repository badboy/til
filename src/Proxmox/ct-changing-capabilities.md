# Dropping sys_rawio capabilities for LXC container

Proxmox can launch leightweight LXC-powered containers.
By default they run unprivileged, meaning root (UID 0) inside the container is mapped to a non-root ID (e.g. UID 100000) on the host (see also [Proxmox Wiki: Unprivileged LXC containers](https://pve.proxmox.com/wiki/Unprivileged_LXC_containers).

Launching Debian in such a container works, but some services might fail to start:

```
; sudo systemctl | grep failed
* sys-kernel-config.mount              loaded failed failed    Kernel Configuration File System
```

Instead of just masking that service (so that it never launches) we can take away the `sys_rawio` capability.
The service then handles it correctly: If the capability is not available it won't even try.

To do that edit `/etc/pve/lxc/$ID.conf`, where `$ID` is the ID of your container, e.g. 102.
Add this line:

```
lxc.cap.drop: sys_rawio
```

Save, restart the container and it should all be fine again.

Source: [Proxmox LXC, Systemd, and Linux Capabilities](https://www.enricobassetti.it/2023/05/proxmox-lxc-systemd-and-linux-capabilities/)
