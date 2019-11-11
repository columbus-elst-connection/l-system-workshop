use api::{LSystem, Symbol};
use clap::{arg_enum, value_t, App, Arg};
use interpreter::Interpreter;
use parser::parse;
use renderer::crab::Crab;
use renderer::{Renderer, StringRenderer};
use std::fmt::Debug;
use std::fs::read_to_string;
use std::hash::Hash;

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum RendererKind {
        Textual,
        Graphical
    }
}

fn main() {
    Crab::global_init();

    let parsed_args = App::new("l-sysem")
        .about("Renders an l-system")
        .arg(
            Arg::with_name("iterations")
                .short("n")
                .long("iterations")
                .takes_value(true)
                .default_value("1")
                .help("The number of token replacement iterations to perform"),
        )
        .arg(
            Arg::with_name("l-system-file")
                .short("f")
                .long("file")
                .takes_value(true)
                .required(true)
                .help("file containing the complete description of the l-system"),
        )
        .arg(
            Arg::with_name("renderer")
                .short("r")
                .long("renderer")
                .takes_value(true)
                .required(false)
                .default_value("textual")
                .possible_values(&RendererKind::variants())
                .case_insensitive(true)
                .help("kind of renderer to use"),
        )
        .get_matches();

    let iterations = parsed_args
        .value_of("iterations")
        .unwrap()
        .parse::<usize>()
        .unwrap_or_else(|_| {
            eprintln!("iterations argument must be a positive integer");
            ::std::process::exit(1);
        });
    let file_name = parsed_args.value_of("l-system-file").unwrap(); // safe unwrap since l-system-file is required
    let input = read_to_string(file_name).unwrap();

    let system = parse(&input).expect("Failed to parse definition of L-system");

    let renderer_kind = value_t!(parsed_args, "renderer", RendererKind).unwrap_or_else(|_| {
        ::std::process::exit(1);
    });

    match renderer_kind {
        RendererKind::Textual => {
            let writer = std::io::stdout();
            let mut renderer = StringRenderer::new(writer);
            render(system, iterations, &mut renderer);
        }
        RendererKind::Graphical => {
            let mut renderer = Crab::new(system.render_config.clone());
            render(system, iterations, &mut renderer);
        }
    };
}

fn render<T: Symbol>(system: LSystem<T>, iterations: usize, renderer: &mut impl Renderer)
where
    T: Copy + Eq + Debug + Hash,
{
    let interpreter = Interpreter::new(system);
    for symbol in interpreter.level(iterations) {
        renderer.render(symbol);
    }
    renderer.finish();
}
