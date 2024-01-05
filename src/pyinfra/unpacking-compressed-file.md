# Download and unpacking a compressed file using pyinfra

[pyinfra] is a tool to automate infrastructure deployments.
Instead of defining everything in YAML like Ansible it uses plain Python code.

I heard about it a while ago and finally started to use it for a single server deployment.

One of the things deployed is a web frontend.
To update it I would download the new `tar.gz` file from the releases page,
extract it and move it to the final location to be used by my webserver.
I wanted to automate that.

pyinfra has an operation to [download files](https://docs.pyinfra.com/en/2.x/operations/files.html#files-download),
but these are just placed in the filesystem.
The file is not extracted nor its contents moved to a specific place.

I thus combined that with some additional shell commands to extract the file and put the content in the final destination.
I'm not sure if this is a bit of a hack or the right way to do things in pyinfra.

First of define the tool version and checksum in `group_data/all.py`:

```python
tool_version = "v1.0.4"
tool_sha256 = "01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b"
```

Then we define the download and extraction step in `deploy.py`.

```python
from pyinfra import host
from pyinfra.operations import files, server

# The URL to download from
tool_url = f"https://tool-homepage.example.com/releases/download/tool-{host.data.tool_version}.tar.gz"
# Where to store the downloaded file
tool_target = "/opt/tool/tool.tar.gz"

tool_downloaded = files.download(
    name=f"Download tool {host.data.tool_version}",
    src=tool_url,
    dest=tool_target,
    sha256sum=host.data.tool_sha256,
)
if tool_downloaded.changed:
    server.shell(
      name="Unpack tool and move content to destination",
      commands=[
        f"tar -C /opt/tool -zxvf {tool_target}",
        f"rsync -a /opt/tool/tool-{host.data.tool_version}/ /var/www/tool/"
      ]
    )
```

Some explanation:

* The version and checksum are defined as data. To update change those.
* This downloads the file to a fixed location.
  That way we can rely on pyinfra doing this only if the file is missing or has the wrong checksum.
  No unnecessary re-downloads.
* Only when the tool is actually downloaded, we extract it. We expect the compressed tar file to have one top-level directory `tool-{version}`.
* We rsync this folder into the final location, in the code above that's `/var/www/tool`.
  Depending on your use case you might want to add `--delete` to delete files from the destination that don't exist in the new version of the tool anymore.
* This doesn't clean up old downloads of the compressed file.
  This could be done with some globbing and skipping the new file in that list.

[pyinfra]: https://pyinfra.com/
