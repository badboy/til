# Strip Markdown syntax

In order to strip Markdown syntax and leave only the plain text output one can patch the [Markdown](https://pypi.org/project/Markdown/) parser:

```python
from markdown import Markdown
from io import StringIO

def unmark_element(element, stream=None):
    if stream is None:
        stream = StringIO()
    if element.text:
        stream.write(element.text)
    for sub in element:
        unmark_element(sub, stream)
    if element.tail:
        stream.write(element.tail)
    return stream.getvalue()

Markdown.output_formats["plain"] = unmark_element

__md = Markdown(output_format="plain")
__md.stripTopLevelTags = False

def strip_markdown(text):
    return __md.convert(text)
```

Then call the `strip_markdown` function:

```python
text = """
# Hello *World*!

[Today I learned](https://fnordig.de/til)
"""

print(strip_markdown(text))
```

This results in:

```
Hello World!
Today I learned
```

(via <https://stackoverflow.com/a/54923798>
