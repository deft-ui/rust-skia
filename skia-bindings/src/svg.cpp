#include "bindings.h"

#include "include/core/SkCanvas.h"
#include "include/svg/SkSVGCanvas.h"


#include "modules/svg/include/SkSVGTypes.h"
#include "modules/svg/include/SkSVGDOM.h"
#include "modules/svg/include/SkSVGNode.h"
#include "modules/svg/include/SkSVGSVG.h"
#include "modules/svg/include/SkSVGRenderContext.h"
#include "modules/skresources/include/SkResources.h"

#include "include/core/SkStream.h"


typedef SkData* (*loadSkData)(const char resource_path[], const char resource_name[], void* context);

typedef SkTypeface* (*loadSkTypeface)(const char resource_path[], const char resource_name[], void* context);

class ImageResourceProvider final : public skresources::ResourceProvider {

private:
    loadSkData _loadCb;
    loadSkTypeface _loadTfCb;
    void* _loadContext;

public:
    ImageResourceProvider(loadSkData loadCb, loadSkTypeface loadTfCb, void* loadContext) {
        _loadCb = loadCb;
        _loadTfCb = loadTfCb;
        _loadContext = loadContext;
    }

    sk_sp<SkData> load(const char resource_path [],
                       const char resource_name []) const {
        return sp(_loadCb(resource_path, resource_name, _loadContext));
    }


    sk_sp<skresources::ImageAsset> loadImageAsset(const char resource_path [],
                                                  const char resource_name [],
                                                  const char /*resource_id*/ []) const {
        auto data = this->load(resource_path, resource_name);
        return skresources::MultiFrameImageAsset::Make(data);
    }


    sk_sp<SkTypeface> loadTypeface(const char name[],
                                   const char url[]) const {
        return sp(_loadTfCb(url, name, _loadContext));
    }

    ~ImageResourceProvider() {}

};


extern "C" SkSVGDOM* C_SkSVGDOM_MakeFromStream(SkStream& stream, SkFontMgr* fontMgr, loadSkData loadCb, loadSkTypeface loadTfCb, void* loadContext) {
    auto provider = sk_make_sp<ImageResourceProvider>(loadCb, loadTfCb, loadContext);
    auto builder = SkSVGDOM::Builder();
    builder.setResourceProvider(provider);
    builder.setFontManager(sp(fontMgr));
    return builder.make(stream).release();
}

extern "C" void C_SkSVGDOM_ref(const SkSVGDOM* self) {
    self->ref();
}

extern "C" void C_SkSVGDOM_unref(const SkSVGDOM* self) {
    self->unref();
}

extern "C" bool C_SkSVGDOM_unique(const SkSVGDOM* self) {
    return self->unique();
}

extern "C" void C_SkSVGDOM_setContainerSize(SkSVGDOM* self, const SkSize& size){
    self->setContainerSize(size);
}

extern "C" SkSVGSVG* C_SkSVGDOM_getRoot(const SkSVGDOM* self){
    return self->getRoot();
}


extern "C" void C_SkSVGSVG_ref(const SkSVGSVG* self) {
    self->ref();
}

extern "C" void C_SkSVGSVG_unref(const SkSVGSVG* self) {
    self->unref();
}

extern "C" bool C_SkSVGSVG_unique(const SkSVGSVG* self) {
    return self->unique();
}

extern "C" SkSize C_SkSVGSVG_intrinsicSize(const SkSVGSVG* self) {
    return self->intrinsicSize(SkSVGLengthContext(SkSize::Make(0, 0)));
}

extern "C" bool C_SkSVGSVG_parseAndSetAttribute(SkSVGSVG* self, const char* name, const char* value){
    return self->parseAndSetAttribute(name, value);
}

extern "C" void C_SkSVGSVG_setX(SkSVGSVG* self, const SkSVGLength x){
    return self->setX(x);
}

extern "C" void C_SkSVGSVG_setY(SkSVGSVG* self, const SkSVGLength y){
    return self->setY(y);
}

extern "C" void C_SkSVGSVG_setWidth(SkSVGSVG* self, const SkSVGLength width){
    return self->setWidth(width);
}

extern "C" void C_SkSVGSVG_setHeight(SkSVGSVG* self, const SkSVGLength height){
    return self->setHeight(height);
}
