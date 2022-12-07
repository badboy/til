# iOS log output from device or simulator

On macOS: Use the `Console` tool.
Alternatively: `idevicesyslog` from `libimobiledevice`.

Install:

```
brew install libimobiledevice
```

Usage:

```
idevicesyslog --process Client
```

`Client` stands in for the process name (`Client` is the name of Firefox iOS).
