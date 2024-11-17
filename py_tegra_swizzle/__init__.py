from . import rust, version


def block_height_mip0(height: int) -> int:
    return rust.block_height_mip0(height)

def mip_block_height(mip_height: int, block_height_mip0: int) -> int:
    return rust.mip_block_height(mip_height, block_height_mip0)

PyBlockDim = rust.PyBlockDim

def get_swizzled_surface_size(
        width: int, height: int, depth: int,
        block_dim: PyBlockDim, block_height_mip0: int,
        bpp: int, mipmap_count: int = 1, layer_count: int = 1
):
    rust.swizzled_surface_size(width, height, depth, block_dim, block_height_mip0, bpp, mipmap_count, layer_count)

__version__ = version.version
VERSION = version.version