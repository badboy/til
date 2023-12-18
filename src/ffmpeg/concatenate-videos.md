# Concatenate videos of the same format

Sometimes you end up with several little cuts of a longer video file
and just want to concatenate those together.
Easy and fast to do with the [concat demuxer](https://ffmpeg.org/ffmpeg-formats.html#concat-1).

List all input files in a file `mylist.txt`:
```
file '/path/to/file1'
file '/path/to/file2'
file '/path/to/file3'
```

Then use the concat demuxer with ffmpeg:

```
ffmpeg -f concat -safe 0 -i mylist.txt -c copy output.mp4
```

(via [StackOverflow](https://stackoverflow.com/questions/7333232/how-to-concatenate-two-mp4-files-using-ffmpeg))
