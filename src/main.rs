struct TimeUp;

#[derive(Debug, Clone)]
struct Timer {
    duration: std::time::Duration,
}

impl Default for Timer {
    fn default() -> Self {
        Timer {
            duration: std::time::Duration::from_secs(25 * 60),
        }
    }
}

impl Timer {
    fn rest() -> Timer {
        Timer {
            duration: std::time::Duration::from_secs(5 * 60),
        }
    }

    fn tick(&mut self) -> Result<(), TimeUp> {
        let one_tick = std::time::Duration::from_secs(1);
        self.duration = self.duration.saturating_sub(one_tick);

        if self.duration != std::time::Duration::ZERO {
            Ok(())
        } else {
            Err(TimeUp)
        }
    }

    fn pretty_print(&self, s: &Screen, r: &Renderer) {
        let total_seconds = self.duration.as_secs_f64();
        let minutes = (total_seconds / 60.0).floor();
        let seconds = total_seconds % 60.0;

        let minute_ones_digit = (minutes % 10.0).floor();
        let minute_tens_digit = (minutes / 10.0).floor();

        let second_ones_digit = (seconds % 10.0).floor();
        let second_tens_digit = (seconds / 10.0).floor();

        let minute_ones_digit: u8 = minute_ones_digit as u8;
        let minute_tens_digit: u8 = minute_tens_digit as u8;

        let second_ones_digit: u8 = second_ones_digit as u8;
        let second_tens_digit: u8 = second_tens_digit as u8;

        // sanity checks
        assert!(minute_ones_digit < 60);
        assert!(minute_tens_digit < 60);
        assert!(second_ones_digit < 60);
        assert!(second_tens_digit < 60);

        let fragments = r.string_fragments(
            minute_tens_digit,
            minute_ones_digit,
            second_tens_digit,
            second_ones_digit,
        );

        let mt = fragments[0].0;
        let mo = fragments[1].0;
        let co = fragments[2].0;
        let st = fragments[3].0;
        let so = fragments[4].0;

        for x in 0..co.len() {
            s.print(format!(
                "{} {} {} {} {}",
                colored::Colorize::bold(mt[x]),
                colored::Colorize::bold(mo[x]),
                colored::Colorize::bold(colored::Colorize::italic(co[x])),
                colored::Colorize::bold(st[x]),
                colored::Colorize::bold(so[x])
            ));
        }
    }
}

struct Screen;

impl Screen {
    fn new() -> Screen {
        Screen
    }

    fn clear(&self) {
        println!();
        print!("{esc}[2J{esc}[1;1H", esc = 27_u8 as char);
        println!();
    }

    fn print(&self, p: String) {
        println!("{}", p);
    }
}

struct Focus {
    screen: Screen,
    renderer: Renderer,
}

impl Focus {
    fn new(s: Screen, r: Renderer) -> Self {
        Focus {
            screen: s,
            renderer: r,
        }
    }

    fn start_session(&self) {
        let timer = Timer::default();
        self.start_timer(timer);
    }

    fn start_rest(&self) {
        let timer = Timer::rest();
        self.start_timer(timer);
    }

    fn start_timer(&self, mut timer: Timer) {
        while timer.tick().is_ok() {
            timer.pretty_print(&self.screen, &self.renderer);
            println!();
            std::thread::sleep(std::time::Duration::from_secs(1));
            self.screen.clear();
        }
    }

    fn notify(&self, msg: &str) {
        notify_rust::Notification::new()
            .summary("Tameta")
            .body(msg)
            .timeout(notify_rust::Timeout::Milliseconds(12_000))
            .show()
            .unwrap();
    }
}

fn main() -> std::process::ExitCode {
    let screen = Screen::new();
    let renderer = Renderer::new();

    let focus = Focus::new(screen, renderer);

    focus.start_session();
    focus.notify("Your focus session has finished, please take a break!");

    focus.start_rest();
    focus.notify("Your break session has ended, time to get back to studying!");

    loop {
        println!(
            "{} Do you want to continue for {} session? {}",
            colored::Colorize::bold(colored::Colorize::red("[tameta]")),
            colored::Colorize::italic("another"),
            colored::Colorize::bold("Y/n")
        );
        let mut input = std::io::stdin().lock();
        let mut buffer = String::new();
        std::io::BufRead::read_line(&mut input, &mut buffer).unwrap();

        if buffer.trim().to_uppercase() == "Y" {
            focus.start_session();
            focus.notify("Your focus session has finished, please take a break!");

            focus.start_rest();
            focus.notify("Your break session has ended, time to get back to studying!");
        } else {
            return std::process::ExitCode::SUCCESS;
        }
    }
}

struct Font<'a>([&'a str; 5]);

impl<'a> Font<'a> {
    fn new(s: [&'a str; 5]) -> Font {
        Font(s)
    }
}

struct Renderer;

impl Renderer {
    fn new() -> Self {
        Renderer
    }

    fn string_fragments(&self, min_ten: u8, min_one: u8, sec_ten: u8, sec_one: u8) -> [Font; 5] {
        let mt_str = self.to_ascii(min_ten);
        let mo_str = self.to_ascii(min_one);
        let colon = self.to_ascii(58);
        let st_str = self.to_ascii(sec_ten);
        let so_str = self.to_ascii(sec_one);

        [mt_str, mo_str, colon, st_str, so_str]
    }

    fn to_ascii(&self, num: u8) -> Font {
        match num {
            0 => Font::new(["  ___  ", " / _ \\ ", "| | | |", "| |_| |", " \\___/ "]),
            1 => Font::new([" _ ", "/ |", "| |", "| |", "|_|"]),
            2 => Font::new([" ____  ", "|___ \\ ", "  __) |", " / __/ ", "|_____|"]),
            3 => Font::new([" _____ ", "|___ / ", "  |_ \\ ", " ___) |", "|____/ "]),
            4 => Font::new([" _  _   ", "| || |  ", "| || |_ ", "|__   _|", "   |_|  "]),
            5 => Font::new([" ____  ", "| ___| ", "|___ \\ ", " ___) |", "|____/ "]),
            6 => Font::new(["  __   ", " / /_  ", "| '_ \\ ", "| (_) |", " \\___/  "]),
            7 => Font::new([" _____ ", "|___  |", "   / / ", "  / /  ", " /_/   "]),
            8 => Font::new(["  ___  ", " ( _ ) ", " / _ \\ ", "| (_) |", " \\___/  "]),
            9 => Font::new(["  ___   ", " / _ \\  ", "| (_) | ", " \\__, | ", "   /_/  "]),
            58 => Font::new([" _ ", "(_)", " _ ", "(_)", "   "]),
            _ => std::panic!(
                "Unable to render requested {num} ({c}) number",
                c = num as char
            ),
        }
    }
}
