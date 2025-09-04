OFX_DIR=$1

# Conditionally derive Deserialize, Serialize for selected structs
# Bindgen output is piped through sed to insert the derive lines
serde_structs="OfxRangeD OfxRangeI OfxRectD OfxRectI OfxPointI OfxPointD OfxTime"
serde_regex=$(echo "$serde_structs" | sed 's/ /\\|/g')

# Uses bool type, parse as C++
bindgen --rust-target 1.73 \
        --with-derive-eq \
        --default-alias-style new_type \
        --generate types,vars \
        --blocklist-type wchar_t \
        --blocklist-type max_align_t \
        --no-debug OfxStatus \
        --allowlist-var 'kOfx.*' \
        --allowlist-type '.*' \
        --no-doc-comments \
        src/openfx-all.h \
        -- -I$OFX_DIR \
        | sed "s/\(pub struct \($serde_regex\)[( ]\)/#[cfg_attr(feature = \"derive_serde\", derive(Deserialize, Serialize))]\n\1/" \
        > src/openfx_all.rs
