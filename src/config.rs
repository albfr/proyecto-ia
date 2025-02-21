pub struct Config {
    show_first: bool,
    help: bool,
    solution_interval: usize,
    level_limit: usize,
    preprocess: bool, // TODO: implement preprocessor
    report_delta: u64,
    randomization_seed: Option<u64>, // TODO: implement randomization
    timeout: Option<u64>,
    verbose: bool, // TODO: implement verbose statistics
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        let mut config = Config {
            show_first: false,
            help: false,
            solution_interval: 0,
            level_limit: 12,
            preprocess: false,
            report_delta: 5,
            randomization_seed: None,
            timeout: None,
            verbose: false,
        };

        let mut args = args.iter();

        while let Some(arg) = args.next() {
            let arg = arg.as_str();

            match arg {
                "--show-first" | "-f" => config.show_first = true,
                "--help" | "-h" => {
                    config.help = true;

                    return Ok(config);
                },
                "--solution-interval" | "-i" => {
                    let i_err_str = "--solution-interval (-i) requires an integer argument";

                    if let Some(n) = args.next() {
                        match n.parse() {
                            Ok(i) => config.solution_interval = i,
                            Err(_) => return Err(i_err_str),
                        }
                    } else {
                        return Err(i_err_str);
                    }
                },
                "--level-limit" | "-l" => {
                    let l_err_str = "--level-limit (-l) requires an integer argument";

                    if let Some(n) = args.next() {
                        match n.parse() {
                            Ok(l) => config.level_limit = l,
                            Err(_) => return Err(l_err_str),
                        }
                    } else {
                        return Err(l_err_str);
                    }
                },
                "--preprocess" | "-p" => config.preprocess = true,
                "--report" | "-r" => {
                    let r_err_str = "--report (-r) requires an integer argument";

                    if let Some(n) = args.next() {
                        match n.parse() {
                            Ok(r) => config.report_delta = r,
                            Err(_) => return Err(r_err_str),
                        }
                    } else {
                        return Err(r_err_str);
                    }
                },
                "--randomize" | "-s" => {
                    let s_err_str = "--randomize (-s) requires an integer argument";

                    if let Some(n) = args.next() {
                        match n.parse() {
                            Ok(s) => config.randomization_seed = Some(s),
                            Err(_) => return Err(s_err_str),
                        }
                    } else {
                        return Err(s_err_str);
                    }
                },
                "--timeout" | "-t" => {
                    let t_err_str = "--timeout (-t) requires an integer argument";

                    if let Some(n) = args.next() {
                        match n.parse() {
                            Ok(t) => config.timeout = Some(t),
                            Err(_) => return Err(t_err_str),
                        }
                    } else {
                        return Err(t_err_str);
                    }
                },
                "--verbose" | "-v" => config.verbose = true,
                _ => (),
            }
        }

        Ok(config)
    }

    pub fn show_first(&self) -> bool {
        self.show_first
    }

    pub fn help(&self) -> bool {
        self.help
    }

    pub fn get_solution_interval(&self) -> usize {
        self.solution_interval
    }

    pub fn get_level_limit(&self) -> usize {
        self.level_limit
    }

    pub fn preprocess(&self) -> bool {
        self.preprocess
    }

    pub fn get_report_delta(&self) -> u64 {
        self.report_delta
    }

    pub fn get_randomization_seed(&self) -> Option<u64> {
        self.randomization_seed
    }

    pub fn get_timeout(&self) -> Option<u64> {
        self.timeout
    }

    pub fn is_verbose(&self) -> bool {
        self.verbose
    }

    pub fn show_help(&self) {
        println!(
"An eXact Cover (XC) solver using Dancing Links (DLX).

Usage: ./dlx [OPTIONS]

Enter via stdin a line of items. These must be unique ASCII strings not having
'|'. This character is a reserved separator of primary and secondary items.

Enter secondary items following '|' if desired.

For example, the following line has 4 primary and 3 secondary items:
a b c d | e f g

Then, again via stdin, enter an option, one per line. An option is a set of
items. These must match the names entered previously and cannot repeat in an
option. Reading of options ends when reaching end-of-file (EOF).

Options:
  -f, --show-first                 Print first solution if it exists
  -h, --help                       Print this help menu
  -i, --solution-interval <SPACE>  Print a solution in intervals of <SPACE>
  -l, --level-limit <LEVEL>        Show up to <LEVEL> braches in reports
  -p, --preprocess                 Apply preprocessing to exact cover problem
  -r, --report <SECS>              Print a report every <SECS> seconds
  -s, --randomize <SEED>           Pick item to cover in a random fashion
  -t, --timeout <SECS>             Stop program execution after <SECS> seconds
  -v, --verbose                    Print verbose output

Default options:
  -f: false (does not mean first solution is not printed, if -i=1 it will),
  -h: false,
  -i: 0 (no solutions are printed by default),
  -l: 12,
  -p: false,
  -r: 5,
  -s: None (first item of minimum length is chosen),
  -t: None,
  -v: false"
        );
    }
}
