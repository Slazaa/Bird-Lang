use crate::bird::constants::compile::PRIMITIVE_PREFIX;

pub fn utils() -> String {
    format!(
        "\
typedef char {PRIMITIVE_PREFIX}i8;\
typedef short {PRIMITIVE_PREFIX}i16;\
typedef long {PRIMITIVE_PREFIX}i32;\
typedef long long {PRIMITIVE_PREFIX}i64;\
\
typedef unsigned int {PRIMITIVE_PREFIX}uint;\
typedef unsigned char {PRIMITIVE_PREFIX}u8;\
typedef unsigned short {PRIMITIVE_PREFIX}u16;\
typedef unsigned long {PRIMITIVE_PREFIX}u32;\
typedef unsigned long long {PRIMITIVE_PREFIX}u64;\
\
typedef float {PRIMITIVE_PREFIX}f32;\
typedef double {PRIMITIVE_PREFIX}f64;\
\
typedef u32 {PRIMITIVE_PREFIX}char;\
\
typedef enum {{ false, true }} {PRIMITIVE_PREFIX}bool;\
"
    )
}
