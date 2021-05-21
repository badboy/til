# Check who holds SecureInput lock

Individual applications on macOS can request `SecureInput` mode, which disables some functionality that would otherwise allow to capture input.
One can check if `SecureInput` is active and which process holds the lock:

```
$ ioreg -l -w 0 | grep SecureInput
  |   "IOConsoleUsers" = ({"kCGSSessionOnConsoleKey"=Yes,"kSCSecuritySessionID"=100024,"kCGSSessionSecureInputPID"=123,"kCGSSessionGroupIDKey"=20,
      "kCGSSessionIDKey"=257,"kCGSessionLoginDoneKey"=Yes,"kCGSSessionSystemSafeBoot"=No,"kCGSSessionUserNameKey"="user",
      "kCGSessionLongUserNameKey"="username","kCGSSessionAuditIDKey"=100001,"kCGSSessionLoginwindowSafeLogin"=No,"kCGSSessionUserIDKey"=101})
```

The `kCGSSessionSecureInputPID` holds the PID of the process that holds the `SecureInput` lock.
Find that process with `ps`:

```
ps aux | grep $pid
```
