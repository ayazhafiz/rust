use crate::spec::{LinkerFlavor, Target, TargetOptions};

pub fn target() -> Target {
    let mut base = super::linux_musl_base::opts();
    base.pre_link_args.get_mut(&LinkerFlavor::Gcc).unwrap().push("-m32".to_string());
    base.max_atomic_width = Some(32);

    Target {
        llvm_target: "powerpc-unknown-linux-musl".to_string(),
        target_endian: "big".to_string(),
        target_pointer_width: "32".to_string(),
        target_c_int_width: "32".to_string(),
        data_layout: "E-m:e-p:32:32-i64:64-n32".to_string(),
        arch: "powerpc".to_string(),
        target_os: "linux".to_string(),
        target_env: "musl".to_string(),
        target_vendor: "unknown".to_string(),
        linker_flavor: LinkerFlavor::Gcc,
        options: TargetOptions { target_mcount: "_mcount".to_string(), ..base },
    }
}
