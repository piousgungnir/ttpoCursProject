fn main() {
    tonic_build::compile_protos("proto/solve_expr.proto").expect("TODO: panic message");
    tauri_build::build();
}
