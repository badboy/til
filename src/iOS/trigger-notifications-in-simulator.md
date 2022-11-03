# Trigger notifications in the simulator

The notification goes into a `notification.apns` file:

```json
{
    "aps": {
        "alert": {
            "title": "Push Notification",
            "subtitle": "Test Push Notifications",
            "body" : "Testing Push Notifications on iOS Simulator",
        }
    }
}
```

Trigger the notification with:

```
xcrun simctl push booted com.apple.MobileSMS notification.apns
```

This sends it to the application `com.apple.MobileSMS`.
Your own application's bundle identifier can be used if it handles notifications.

The application's bundle identifier can also be specified in the APNS file with the `"Simulator Target Bundle"` key.
It can be left out on the command-line in that case.

APNS files can also be dropped on the simulator to be send.
