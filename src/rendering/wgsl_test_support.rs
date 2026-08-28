#![cfg(test)]

pub(crate) fn parsed_wgsl(name: &str, src: &str) -> naga::Module {
    naga::front::wgsl::parse_str(src)
        .unwrap_or_else(|err| panic!("{name}.wgsl parse: {}", err.emit_to_string(src)))
}

fn wgsl_struct_layout(module: &naga::Module, name: &str) -> (Vec<(String, u32)>, u32) {
    for (_, ty) in module.types.iter() {
        if ty.name.as_deref() == Some(name)
            && let naga::TypeInner::Struct { members, span } = &ty.inner
        {
            let fields = members
                .iter()
                .map(|member| (member.name.clone().unwrap_or_default(), member.offset))
                .collect();
            return (fields, *span);
        }
    }
    panic!("struct {name} not found in shader");
}

pub(crate) fn assert_uniform_layout(
    shader: &str,
    src: &str,
    wgsl_struct: &str,
    rust_fields: &[(&str, usize)],
    rust_size: usize,
) {
    let module = parsed_wgsl(shader, src);
    let (members, span) = wgsl_struct_layout(&module, wgsl_struct);
    assert_eq!(span as usize, rust_size, "{shader} {wgsl_struct} size");
    assert_eq!(members.len(), rust_fields.len(), "{shader} {wgsl_struct} count");
    for ((wgsl_name, wgsl_off), (rust_name, rust_off)) in members.iter().zip(rust_fields) {
        assert_eq!(wgsl_name, rust_name, "{shader} {wgsl_struct} order");
        assert_eq!(*wgsl_off as usize, *rust_off, "{shader} {wgsl_struct}.{wgsl_name} offset");
    }
}
