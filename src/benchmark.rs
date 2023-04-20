pub struct Benchmark {
    /// Alias of the benchmark function.
    pub name: String,
    /// Module path (e.g. `path::to::fn`)
    pub fn_path: String,

    pub args: Vec<BenchParam>,
}

pub struct BenchParam {
    /// Name of the parameter.
    pub name: String,
    /// Type of the parameter.
    pub ty: String,
    /// The [`Display`] representation of the value.
    pub val: String,
}
