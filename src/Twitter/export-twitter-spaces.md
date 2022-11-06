# Exporting Twitter Spaces recording

For better or for worse people are using Twitter Spaces more and more: audio-only conversations on Twitter.
Simon Willison recently hosted one and [wrote a TIL how to download it](https://til.simonwillison.net/twitter/export-edit-twitter-spaces).
I helped because it's actually easier than his initial solution, so I'm copying that here:

## Exporting the recording  using youtube-dl

Open the Twitter Spaces page, open your Firefox developer tools console in the network tab,
filter for "m3u", then hit "Play" on the page.
The network tab will capture the URL to the playlist file. Copy that.

Then use `youtube-dl` (or one of its more recent forks like [yt-dlp](https://github.com/yt-dlp/yt-dlp)) to download the audio:

```
youtube-dl "https://prod-fastly-us-west-1.video.pscp.tv/Transcoding/v1/hls/GPI6dSzgZcfqRfMLplfNp_0xu1QXQ8iDEEA0KymUd5WuqOZCZ9LGGKY6vBQdumX7YV1TT2fGtMdXdl2qqtVvPA/non_transcode/us-west-1/periscope-replay-direct-prod-us-west-1-public/audio-space/playlist_16798763063413909336.m3u8?type=replay"
```

This will result in a `.mp4` file (media container):

```
$ mediainfo "playlist_16798763063413909336 [playlist_16798763063413909336].mp4"
General
Complete name                            : playlist_16798763063413909336 [playlist_16798763063413909336].mp4
Format                                   : ADTS
Format/Info                              : Audio Data Transport Stream
[...]
Audio
Format                                   : AAC LC
Format/Info                              : Advanced Audio Codec Low Complexity
[...]
```

To extract only the audio part you can use ffmpeg:

```
ffmpeg -i "playlist_16798763063413909336 [playlist_16798763063413909336].mp4" -vn -acodec copy twitter-spaces-recording.aac
```
