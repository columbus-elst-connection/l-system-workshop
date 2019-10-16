pub mod crab;

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
    use super::*;

    #[test]
    fn string_renderer_prints_each_symbol() {
        let input = "ABCDEFGABCDEFG";
        let mut output = Vec::new();
        {
            let mut renderer = StringRenderer::new(&mut output);
            for symbol in input.chars() {
                renderer.render(symbol);
            }
            renderer.finish();
        }
        let result_string = String::from_utf8(output).unwrap();
        assert_eq!(input, &result_string);
    }
}
