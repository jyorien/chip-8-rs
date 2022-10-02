mod emulator;

const FILE_NAME: &str = "../resources/c8games/IBM";
fn main() {
    let mut chip8 = emulator::Emulator::new();
    chip8.load_rom(FILE_NAME);
}
