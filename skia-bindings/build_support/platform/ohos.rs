use super::prelude::*;

pub struct Ohos;

impl PlatformDetails for Ohos {
    fn uses_freetype(&self, _config: &BuildConfiguration) -> bool {
        false
    }

    fn gn_args(&self, config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
        gn_args(config, builder)
    }

    fn link_libraries(&self, features: &Features) -> Vec<String> {
        link_libraries(features)
    }
}

pub fn gn_args(config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
    builder.target_os_and_default_cpu(&config.target.system);
}

pub fn link_libraries(features: &Features) -> Vec<String> {
    let mut libs = vec!["c++_static"];
    if features.gl {
        libs.extend(vec!["EGL", "GLESv2"])
    };
    libs.into_iter().map(|s| s.to_string()).collect()
}
