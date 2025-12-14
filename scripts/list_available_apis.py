#!/usr/bin/env python3
import os
from pathlib import Path

# Folders in the /usr/include directory of the sysroot which are not relevant for OpenHarmony bindings,
# since they contain just standard headers
blocklisted = [
    'EGL', 'GL', 'GLES2', 'GLES3',
    'KHR',
    'aarch64-linux-ohos',
    'arm-linux-ohos',
    'asm-generic', 'asm-loongarch', 'asm-mips', 'asm-riscv',
    'arpa',
    'drm',
    'fortify',
    'linux',
    'misc',
    'mtd',
    'net', 'netinet', 'netpacket',
    'rdma',
    'scsi',
    # not sure about SLES, it also has  an OH specific header file.
    'SLES',
    'sound',
    'sys',
    'unicode',
    'uv',
    'video',
    'vk_video',
    'vulkan',
    'i686-linux-ohos',
    'x86_64-linux-ohos',
    'xen',
]

if __name__ == '__main__':
    native_path = Path(os.environ['OHOS_SDK_NATIVE'])
    header_root = native_path.joinpath('sysroot/usr/include')
    assert header_root.exists(), f"{header_root} does not exist"
    dirs = [x.parts[-1] for x in header_root.iterdir() if x.is_dir()]
    dirs.sort()
    filtered = list(filter(lambda name: name not in blocklisted, dirs))

    print("\n".join(filtered))