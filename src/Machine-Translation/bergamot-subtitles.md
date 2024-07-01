# Using machine translation for subtitles in mpv

The Bergamot project built a small and fast machine translation model that runs completely local on your machine.
Mozilla helped turn this into an addon[^1], [a website][moztranslate]
and [integrated it into Firefox][ffxtranslate].
You can also go to <about:translations> directly.

The Bergamot translator is [available as a C++ library][cpp],
which can also be compiled to WebAssembly[^2].

I wanted to use it to translate subtitles in a movie on-the-fly.
The movie only contained subtitles in languages I barely know (Norwegian[^3], Swedish, Finnish, Danish),
so getting some help with those subtitles translated to English is required.

## Getting Bergamot Translator to work

Getting a CLI tool to use Bergamot translator is [properly documented][bergamot-docs],
but of course I had to run into issues first before reading that and figuring out.

1. Clone the repository
2. Configure and build the project

```
; git clone https://github.com/browsermt/bergamot-translator
; cd bergamot-translator
; mkdir build
; cd build
; cmake ..
; make -j8
```

That will give you a CLI tool in `build/app/bergamot`:

```
; build/app/bergamot --build-info
AVX2_FOUND=false
AVX512_FOUND=false
AVX_FOUND=false
BUILD_ARCH=native
<snip>
```

which just works:

```
; build/app/bergamot
[1]    22529 segmentation fault  build/app/bergamot
```

or not.

You need the models from <http://data.statmt.org/bergamot/models/> first.
I wanted to translate Norwegian, so went with `nben`[^4]. That is this file:

<https://data.statmt.org/bergamot/models/nben/nben.student.tiny11.v1.e410ce34f8337aab.tar.gz>

```
; mkdir -p models
; wget --quiet --continue --directory models/ \
    https://data.statmt.org/bergamot/models/nben/nben.student.tiny11.v1.e410ce34f8337aab.tar.gz
; (cd models && tar -xzf nben.student.tiny11.v1.e410ce34f8337aab.tar.gz)
```

Then some patching is required, Bergamot has a tool for that:

```
; python3 bergamot-translator-tests/tools/patch-marian-for-bergamot.py --config-path models/nben.student.tiny11/config.intgemm8bitalpha.yml --ssplit-prefix-file $(realpath 3rd_party/ssplit-cpp/nonbreaking_prefixes/nonbreaking_prefix.en)
```

I don't actually know what exactly it patches nor why they don't offer the already patched files.

Last but not least the tool now translates text on stdin from Norwegian to English:

```
; CONFIG=models/nben.student.tiny11/config.intgemm8bitalpha.yml.bergamot.yml
; build/app/bergamot --model-config-paths $CONFIG --cpu-threads 4 <<< "Jeg snakker litt norsk"
I'm talking a little Norwegian.
```

For ease of use later I wrapped this in a short shell script:

```
; cat ~/bin/translate-nben
#!/bin/bash

CONFIG=~/code/bergamot-translator/models/nben.student.tiny11/config.intgemm8bitalpha.yml.bergamot.yml
printf "%s" "$1" | ~/code/bergamot-translator/build/app/bergamot --model-config-paths $CONFIG --cpu-threads 4
```

Make it executable, then run it with the text to translate as an argument:

```
; chmod +x ~/bin/translate-nben
; translate-nben "Jeg snakker litt norsk."
I speak a little Norwegian.
```

## Translating subtitles in mpv

I found a script for mpv[^5] that does the heavy lifting of extracting the current subtitle line,
sending it to a translator and displaying the answer: [subtitle-translate-mpv].
It uses [Crow], another translator tool that uses a number of web services for the translation.We replace that later with our local-only Bergamot-powered translator.

Let's install the script (this is on macOS, adjust paths accordingly for other systems)

```
; cd ~/.config/mpv
; mkdir -p scripts && cd scripts
; git clone --depth 1 https://github.com/EnergoStalin/subtitle-translate-mpv.git
```

Add the configuration for hotkeys:

```
; cat ~/.config/mpv/input.conf
CTRL+t script-message enable-sub-translator
CTRL+T script-message disable-sub-translator
ALT+t script-message sub-translated-only
ALT+o script-message sub-primary-original
```

Now edit `~/.config/mpv/scripts/subtitle-translate-mpv/modules/translators/crow.lua`.
Find the line where it puts together the command to run.
At the time of this writing [this is line 35 and following](https://github.com/EnergoStalin/subtitle-translate-mpv/blob/8224172179ebd829a1000704f375db5f047c157e/modules/translators/crow.lua#L35-L40).

Change that to run your script instead:

```lua
local args = {
	'/Users/jer/bin/translate-nben',
	escaped
}
```

Save this and you're set.

Start your movie, enable subtitles (press `v` and select the right language with `j`).
Then press `Ctrl-t` to enable the translation (disable it again with `Ctrl-T`, so that's `Ctrl-Shift-t`).
Toggle the original one on or off with `Option-t` (or `Alt-t` on not-macOS).

This is what it will look like, original subtitle on the bottom, translated one above[^6]:

<figure>
  <img src="nach-screenshot-with-subtitles.jpg" alt="The two main characters, Selma and Elin. Norwegian subtitle on the bottom: Ok. Det går fint, altså. Jeg kan sove på en madrass. English translation in a bigger font above that: Okay, okay, okay, okay. I mean, it's okay. I can sleep on a mattress.">
  <figcaption>
  Scene from <a href="https://www.imdb.com/title/tt14534548/">Nach</a> S01 E01, at 00:02:54.
  </figcaption>
</figure>

---

[^1]: Now part of Firefox directly.

[^2]: That's what the website and the Firefox integration use.

[^3]: [365 day learning streak now!](https://hachyderm.io/@jer/111691269347746755)

[^4]: nb=Norwegian Bokmål, one of the official written standards of the Norwegian language; en=English.

[^5]: [mpv.io][mpv], my media player of choice.

[^6]: I have mine configured to be a bit smaller, `sub-scale=0.5` in `~/.config/mpv/mpv.conf`.

[bergamot]: https://browser.mt/
[moztranslate]: https://mozilla.github.io/translate/
[ffxtranslate]: https://www.mozilla.org/en-US/firefox/features/translate/
[cpp]: https://github.com/browsermt/bergamot-translator
[bergamot-docs]: https://browser.mt/docs/main/marian-integration.html#building-bergamot-translator
[mpv]: https://mpv.io
[subtitle-translate-mpv]: https://github.com/EnergoStalin/subtitle-translate-mpv
[nach]: https://www.imdb.com/title/tt14534548/
[crow]: https://crow-translate.github.io/
