use std::env;

fn main() {
    let cuda_path_defined = env::var("CUDA_PATH").is_ok();

    if cuda_path_defined {
        println!("cargo:warning=CUDA_PATH found, enableling CUDA support");
        println!("cargo:rerun-if-env-changed=CUDA_PATH");
        println!("cargo:rustc-cfg=feature=\"cuda\"");
    } else {
        println!("cargo:note=CUDA_PATH not found. If you want CUDA support, please define CUDA_PATH and enable the 'cuda_support' feature.");
    }
}
