# Symbols in shared libraries

To look at the largest objects/functions in libxul:

```
readelf -sW $NIGHTLY/libxul.so | sort -k 3 -g -r | head -n 100
```

To look at the disassembly:

```
objdump -dr $OBJ | c++filt
```

On macOS:

```
otool -tV $OBJ | c++filt
```
