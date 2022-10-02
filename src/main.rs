mod emulator;

const FILE_NAME: &str = "../resources/c8games/IBM";
fn main() {
    let result = emulator::Emulator::loadROM(FILE_NAME).unwrap();
    println!("{:?}", result);
}
