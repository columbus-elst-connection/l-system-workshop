use api::{RendererInstruction, Symbol};
use std::io::Write;

pub trait Renderer {
    fn global_init() where Self: Sized {}

    fn render(&mut self, symbol: impl Symbol) {
        let instruction = symbol.to_rendering_instruction();

        match instruction {
            RendererInstruction::Push => self.push(),
            RendererInstruction::Pop => self.pop(),
            RendererInstruction::Forward => self.forward(),
            RendererInstruction::RotateLeft => self.rotate_left(),
            RendererInstruction::RotateRight => self.rotate_right(),
            RendererInstruction::NoOp => { /* no-op */ }
        }
    }

    fn push(&mut self) {}
    fn pop(&mut self) {}

    fn forward(&mut self) {}
    fn rotate_left(&mut self) {}
    fn rotate_right(&mut self) {}

    fn finish(&mut self) {}
}

pub struct StringRenderer<T: Write> {
    writer: T,
}

impl <T: Write> StringRenderer<T> {
    pub fn new(writer: T) -> Self {
        Self {
            writer,
        }
    }
}

impl <T: Write> Renderer for StringRenderer<T> {

    fn render(&mut self, symbol: impl Symbol) {
        write!(&mut self.writer, "{}", symbol).unwrap();
    }

    fn finish(&mut self) {
        self.writer.flush().unwrap();
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
