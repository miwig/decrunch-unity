#include <cstdint>
#include "crnlib/crn_decomp.cpp"

extern "C" {
bool crnd_get_level_info(const void *pData, crnd::uint32 data_size,
                         crnd::uint32 level_index,
                         crnd::crn_level_info *pLevel_info) {
  return crnd::crnd_get_level_info(pData, data_size, level_index, pLevel_info);
}

bool crnd_get_texture_info(const void *pData, crnd::uint32 data_size,
                           crnd::crn_texture_info *pTexture_info) {
  return crnd::crnd_get_texture_info(pData, data_size, pTexture_info);
}

void *crnd_unpack_begin(const void *pData, crnd::uint32 data_size) {
  return crnd::crnd_unpack_begin(pData, data_size);
}

bool crnd_unpack_end(crnd::crnd_unpack_context pContext) {
  return crnd::crnd_unpack_end(pContext);
}

bool crnd_unpack_level(crnd::crnd_unpack_context pContext, void **ppDst,
                       crnd::uint32 dst_size_in_bytes,
                       crnd::uint32 row_pitch_in_bytes,
                       crnd::uint32 level_index) {
  return crnd::crnd_unpack_level(pContext, ppDst, dst_size_in_bytes,
                                 row_pitch_in_bytes, level_index);
}
}
