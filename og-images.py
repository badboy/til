#!/usr/bin/env python
# encoding: utf-8

import urllib
import re
import subprocess
from pathlib import Path
from bs4 import BeautifulSoup

BASE_URL = "https://fnordig.de/til"
DISALLOWED_RE = re.compile("[^a-zA-Z0-9_-]")
CLEANUP_JS = """
document.getElementsByTagName("nav")[1].style.display = "none";
document.querySelectorAll("div.buttons").forEach(e => e.style.display = "none")
"""


def filename_for_url(url, ext=None):
    ext = ext or "png"
    bits = urllib.parse.urlparse(url)
    filename = (bits.netloc + bits.path).replace(".", "-").replace("/", "-").rstrip("-")
    # Remove any characters outside of the allowed range
    base_filename = DISALLOWED_RE.sub("", filename).lstrip("-")
    filename = base_filename + "." + ext
    return filename


def remove_og(soup):
    for meta_tag in soup.head.find_all("meta"):
        property = meta_tag.get("property")
        if property and property.startswith("og:"):
            meta_tag.extract()


def add_og(soup, title, image, desc=None):
    new_title = soup.new_tag("meta", property="og:title", content=title)
    new_desc = soup.new_tag("meta", property="og:description", content=desc)
    new_image = soup.new_tag("meta", property="og:image", content=image)

    soup.head.append(new_title)
    if desc:
        soup.head.append(new_desc)
    soup.head.append(new_image)


def main():
    base = Path("_book")
    pages = list(base.glob("*/*.html"))

    for page in pages:
        uri = BASE_URL + str(Path("/") / page.parts[1] / page.parts[2])
        fn = filename_for_url(uri)
        print(uri, fn)

        output = base / "images" / fn
        image_uri = BASE_URL + str(Path("/images") / fn)

        cmd = [
            "shot-scraper",
            "shot",
            "--width",
            "600",
            "--height",
            "315",
            "--retina",
            "--javascript", CLEANUP_JS,
            "--output",
            output,
            uri,
        ]
        subprocess.run(cmd)

        with open(page) as fp:
            doc = fp.read()
            soup = BeautifulSoup(doc, "html.parser")

        remove_og(soup)
        title = soup.head.title.string
        add_og(soup, title, image_uri)

        with open(page, "w") as fp:
            fp.write(str(soup))


if __name__ == "__main__":
    main()
