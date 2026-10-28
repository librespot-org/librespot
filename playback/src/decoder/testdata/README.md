These FLAC fixtures contain generated audio, with no recorded music.

The decoder accepts sample rates from 8,000 through 384,000 Hz to bound
resampling resources. `stereo-1.flac` verifies that a real file below this
range is rejected before allocating resampled audio. It contains two stereo
frames at 1 Hz and was generated with:

```sh
ffmpeg -f lavfi -i 'anullsrc=r=1:cl=stereo' -t 2 -c:a flac \
  -map_metadata -1 -fflags +bitexact -flags:a +bitexact stereo-1.flac
```

`stereo-44100.flac` is two seconds of stereo silence at 44.1 kHz, generated
with FFmpeg's `anullsrc` filter and `-t 2 -c:a flac`.

`stereo-48000.flac` is exactly 250 ms of two sine waves at 48 kHz. Recreate it
with:

```sh
ffmpeg -f lavfi \
  -i 'aevalsrc=0.25*sin(2*PI*440*t)|0.25*sin(2*PI*880*t):s=48000:d=0.25' \
  -c:a flac -map_metadata -1 -fflags +bitexact -flags:a +bitexact stereo-48000.flac
```

The Opus fixtures are original two-second sine waves generated with FFmpeg:

```sh
ffmpeg -f lavfi -i sine=frequency=440:sample_rate=48000:duration=2 -ac 2 -c:a libopus -b:a 48k -metadata 'title=Opus Test' -metadata 'artist=Test Artist' -metadata 'album=Local Fixtures' stereo-48000.opus
ffmpeg -f lavfi -i sine=frequency=660:sample_rate=48000:duration=2 -ac 1 -c:a libopus -b:a 32k -metadata 'title=Mono Opus' -metadata 'artist=Test Artist' mono-48000.opus
```
