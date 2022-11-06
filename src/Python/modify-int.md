# Modify integer literals

Integer literals in Python refer to the same object every time they are used.
One can modify those objects:

```python
from sys import getsizeof
from ctypes import POINTER, c_void_p, c_char, cast

def read_int(obj: int, vv=True) -> bytes:
    size = getsizeof(obj)
    ptr = cast(c_void_p(id(obj)), POINTER(c_char))
    buf = ptr[0:size]
    if vv:
        print(f"int obj @ {hex(id(obj))}: {buf.hex(' ')}")
    return buf

def write_int(dst: int, src: int):
    raw_src = read_int(src, False)
    dst_ptr = cast(c_void_p(id(dst)), POINTER(c_char))

    for (idx, c) in enumerate(raw_src):
        dst_ptr[idx] = c

read_int(1)
write_int(1, 2)
read_int(1)

a = 1
b = 2
print(a + b)
```

(via <https://twitter.com/segfault_witch/status/1512160978129068032>)
