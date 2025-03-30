OFX_DIR=$1

# Uses bool type, parse as C++
bindgen --rust-target 1.73 \
        --raw-line '#![allow(non_snake_case, non_upper_case_globals)]' \
        --with-derive-eq \
        --default-alias-style new_type \
        --generate types,vars \
        --blocklist-type wchar_t \
        --blocklist-type max_align_t \
        --allowlist-var 'kOfx.*' \
        --allowlist-type '.*' \
        --no-doc-comments \
        src/openfx-all.h \
        -- -I$OFX_DIR -x c++ \
        > src/openfx_all.rs
