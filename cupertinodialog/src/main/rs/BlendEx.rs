#pragma version(1)
#pragma rs_fp_relaxed
#pragma rs java_package_name(com.hanter.android.radwidget.cupertino)

#include "rs_graphics.rsh"

static float4 overlayColor;

void setOverlayColor(int overlay) {
    overlayColor = rsUnpackColor8888(overlay);
}

static float blendOverlayF(float base, float blend) {
    return (base < 0.5 ? (2.0 * base * blend) : (1.0 - 2.0 * (1.0 - base) * (1.0 - blend)));
}

static float3 blendOverlayPointF3(float3 base, float3 blend) {
    float3 blendOverLayPixel = {blendOverlayF(base.r, blend.r), blendOverlayF(base.g, blend.g), blendOverlayF(base.b, blend.b)};
    return blendOverLayPixel;
}

uchar4 RS_KERNEL blendOverlay(uchar4 in, uint32_t x, uint32_t y) {
    float4 f4 = rsUnpackColor8888(in);
    float3 color = blendOverlayPointF3(f4.rgb, overlayColor.rgb);
    uchar4 outc = rsPackColorTo8888(color.r, color.g, color.b, overlayColor.a);
    return outc;
}

