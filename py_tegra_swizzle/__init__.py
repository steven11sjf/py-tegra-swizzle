from . import rust, version


def block_height_mip0(height: int) -> int:
    return rust.block_height_mip0_py(height)


__version__ = version.version
VERSION = version.version