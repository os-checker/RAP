use rustc_span::def_id::DefId;

pub fn should_check(def_id: DefId) -> bool {
    let mut def_str = format!("{:?}", def_id);
    if let Some(x) = def_str.rfind("::") {
        def_str = def_str.get((x + "::".len())..).unwrap().to_string();
    }
    if def_str.contains("drop") {
        return false;
    }
    if def_str.contains("dealloc") {
        return false;
    }
    if def_str.contains("release") {
        return false;
    }
    if def_str.contains("destroy") {
        return false;
    }
    true
}
