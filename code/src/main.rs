use interpreter::Interpreter;
use parser::parse;
use api::{LSystem, Symbol};
use renderer::{ Renderer, StringRenderer };
use renderer::crab::Crab; // Uncomment to use the Crab renderer
use std::fmt::Debug;
use std::hash::Hash;
use std::fs::{read_to_string};
use clap::{App, Arg};

fn main() {
    // Uncomment to use the Crab renderer
    Crab::global_init();

    let parsed_args = App::new("l-sysem")
            .about("Renders an l-system")
            .arg(Arg::with_name("iterations")
                    .short("n")
                    .long("iterations")
                    .takes_value(true)
                    .default_value("1")
                    .help("The number of token replacement iterations to perform"))
            .arg(Arg::with_name("l-system-file")
                    .short("f")
                    .long("file")
                    .takes_value(true)
                    .required(true)
                    .help("file containing the complete description of the l-system"))
            .get_matches();

    let iterations = parsed_args.value_of("iterations").unwrap().parse::<usize>().unwrap_or_else(|_| {
        eprintln!("iterations argument must be a positive integer");
        ::std::process::exit(1);
    });
    let file_name = parsed_args.value_of("l-system-file").unwrap(); // safe unwrap since l-system-file is required
    let input = read_to_string(file_name).unwrap();

    let system = parse(&input)
        .expect("Failed to parse definition of L-system");

    let writer = std::io::stdout();
    let mut renderer = StringRenderer::new(writer);
    // uncomment to use the crab renderer
    let mut renderer = Crab::new(system.render_config.clone());

    render(system, iterations, &mut renderer);
}

fn render<T: Symbol>(system: LSystem<T>, iterations: usize, renderer: &mut impl Renderer) where T: Copy + Eq + Debug + Hash {
    let interpreter = Interpreter::new(system);
    for symbol in interpreter.level(iterations) {
        renderer.render(symbol);
    }
    renderer.finish();
}
