pub type ComponentType = ();
pub type ComponentResult = Result<(), ()>;
pub type Res = Result<(), ()>;

pub trait TToolchainExecutionService {
    fn get_component(&self, toolchain: String, component: String) -> ComponentResult;

    // пакет дергает этот метод при исполнении компонента тулчейна (прописаны не все аргументы)
    fn emit(&self, toolchain: String, component: ComponentType) -> Res;
}
