pub struct Config {
    show_first: bool,
    help: bool, // TODO: implement help menu
    solution_interval: usize,
    level_limit: usize,
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

    pub fn is_help(&self) -> bool {
        self.help
    }

    pub fn get_solution_interval(&self) -> usize {
        self.solution_interval
    }

    pub fn get_level_limit(&self) -> usize {
        self.level_limit
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
}
