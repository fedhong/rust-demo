// no_mangle： 禁用对符号(symbol)名编码，表示在Javascript中依然采用fbin函数名调用本函数。
// extern关键字表示该函数可以在Javascript中调用。
#[no_mangle]
pub extern "C" fn fbin(x: i32) -> i32 {
    if x <= 1 {
        return 1;
    } else {
        return fbin(x - 1) + fbin(x - 2);
    }
}

// 将rust编译为wasm
// cargo build --release --target wasm32-unknown-unknown

// 安装交叉编译器
// rustup target add wasm32-unknown-unknown

// wasm32-unknown-unknown。此⽬标直接使⽤ llvm 后端编译成 wasm。它适合纯 rust 代码编译，譬如你没有 C 依赖的时候。跟 emen ⽬标⽐起来，它默认就⽣成更加洗练的代码， ⽽且也便于设置搭建。
// wasm32-unknown-emen。此⽬标利⽤ emen ⼯具链编译成 wasm。当你具有 C 依赖的时候就得使⽤它了，包括 libc。
