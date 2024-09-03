# Disable the Pixel Launcher

As I was dealing with an issue on my Android test phone the other day
I wanted to disable the default-installed Pixel Launcher.

To do this you run:

```
adb shell pm disable-user --user 0 com.google.android.apps.nexuslauncher
```

If you have no alternative launcher installed this will get your phone stuck without a launcher.
You might see this in `logcat`:

```
D  User unlocked but no home; let's hope someone enables one soon?
```

To re-enable the launcher run:

```
adb shell pm enable com.google.android.apps.nexuslauncher
```
