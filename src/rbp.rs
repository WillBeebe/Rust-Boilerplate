#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_macros)]

extern crate ncurses;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use std::io::Read;
use std::io::Write;

// List comprehension macro
macro_rules! compr {
    ($id1:ident | $id2:ident <- [$start:expr ; $end:expr] , $cond:expr) => {{
        let mut vec = Vec::new();
        for num in $start..=$end {
            if $cond(num) {
                vec.push(num);
            }
        }
        vec
    }};
}

// Vector initialisation macro
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

// Prompts the user for input and returns it as a string.
pub fn input(prompt: &str) -> String {
    print!("\n{} > ", prompt);
    std::io::stdout().flush().unwrap();
    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer).unwrap();
    answer.pop();
    answer
}

// Clears the terminal window on SOME terminal emulators.
pub fn cls() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

// Executes slices as unix shell commands.
pub fn unix_shell(x: &str) {
    let mut commands = x.trim().split(" | ").peekable();
    let mut previous_command = None;

    while let Some(command) = commands.next() {
        let mut parts = command.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = std::path::Path::new(new_dir);
                if let Err(e) = std::env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }

                previous_command = None;
            }
            "exit" => return,
            command => {
                let stdin = previous_command.map_or(
                    std::process::Stdio::inherit(),
                    |output: std::process::Child| std::process::Stdio::from(output.stdout.unwrap()),
                );

                let stdout = if commands.peek().is_some() {
                    std::process::Stdio::piped()
                } else {
                    std::process::Stdio::inherit()
                };

                let output = std::process::Command::new(command)
                    .args(args)
                    .stdin(stdin)
                    .stdout(stdout)
                    .spawn();

                match output {
                    Ok(output) => {
                        previous_command = Some(output);
                    }
                    Err(e) => {
                        previous_command = None;
                        eprintln!("{}", e);
                    }
                };
            }
        }
    }
    if let Some(mut final_command) = previous_command {
        final_command.wait().ok();
    }
}

// Seeds the pseudo-random number generator with unix time.
pub fn seed() {
    let d = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap();
    let _rng = StdRng::seed_from_u64(d.as_secs());
}

// Generates a pseudo-random number between x and y.
pub fn pseudo(x: i32, y: i32) -> i32 {
    rand::thread_rng().gen_range(x..y + 1)
}

// Removes all empty strings from a vector of strings.
pub fn remove_null_strings(v: Vec<String>) -> Vec<String> {
    v.into_iter().filter(|n| n != "").collect::<Vec<_>>()
}

// Returns the nth char (zero indexed) from a slice.
pub fn nth_char(x: &str, n: usize) -> char {
    x.chars().nth(n).unwrap()
}

// Checks if a slice consists only of digits.
pub fn is_digits(x: &str) -> bool {
    x.chars().all(char::is_numeric)
}

// Checks if a slice represents an i32 integer.
pub fn is_i32(x: &str) -> bool {
    x.parse::<i32>().is_ok()
}

// Checks if a slice represents an f64 float.
pub fn is_f64(x: &str) -> bool {
    x.parse::<f64>().is_ok()
}

// Like the "is_f64" function, but with additional format restrictions
pub fn is_number(x: &str) -> bool {
    if !is_f64(x) {
        return false;
    }

    if nth_char(x, 0) == '+' {
        return false;
    }

    if x.len() > 1 && nth_char(x, 0) == '0' {
        return false;
    }

    if x.len() > 1 && nth_char(x, 0) == '-' && nth_char(x, 1) == '0' {
        return false;
    }

    true
}

// Returns the type of variable T.
pub fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

// Converts a slice to an i32 integer.
pub fn to_i32(x: &str) -> i32 {
    x.parse::<i32>().unwrap()
}

// Converts a slice to an f64 float.
pub fn to_f64(x: &str) -> f64 {
    x.parse::<f64>().unwrap()
}

// Counts the number of chars in a slice.
pub fn char_count(x: &str) -> usize {
    x.chars().count()
}

// Counts the number of substrings in a slice, as delimited by a given char.
pub fn slice_count(x: &str, y: char) -> usize {
    if x == "" {
        return 0;
    }
    let mut count: usize = 0;
    let slice_vec = x.split(y);
    for _ in slice_vec {
        count += 1;
    }
    count
}

// Counts the number of words in a slice.
pub fn word_count(x: &str) -> usize {
    if x == "" {
        return 0;
    }
    let mut count: usize = 0;
    let word_vec = x.split(" ");
    for _ in word_vec {
        count += 1;
    }
    count
}

// Counts the number of lines in a slice.
pub fn line_count(x: &str) -> usize {
    if x == "" {
        return 0;
    }
    let mut count: usize = 0;
    let line_vec = x.split("\n");
    for _ in line_vec {
        count += 1;
    }
    count
}

// Returns the nth slice (zero indexed) from a larger slice, as delimited by a given char.
pub fn nth_slice(x: &str, y: usize, z: char) -> String {
    if x == "" || y >= slice_count(x, z) {
        return "".to_string();
    }
    let slice_vec = x.split(z);
    let mut result = "";
    let mut count: usize = 0;
    for r in slice_vec {
        if y == count {
            result = r;
        }
        count += 1;
    }
    result.to_string()
}

// Returns the nth word (zero indexed) from a slice.
pub fn nth_word(x: &str, y: usize) -> String {
    if x == "" || y >= word_count(x) {
        return "".to_string();
    }
    let word_vec = x.split(" ");
    let mut result = "";
    let mut count: usize = 0;
    for r in word_vec {
        if y == count {
            result = r;
        }
        count += 1;
    }
    result.to_string()
}

// Returns the nth line (zero indexed) from a slice.
pub fn nth_line(x: &str, y: usize) -> String {
    if x == "" || y >= line_count(x) {
        return "".to_string();
    }
    let line_vec = x.split("\n");
    let mut result = "";
    let mut count: usize = 0;
    for r in line_vec {
        if y == count {
            result = r;
        }
        count += 1;
    }
    result.to_string()
}

// Writes data to a file.
pub fn write_to_file(path: &str, data: &str) {
    std::fs::write(path, data).unwrap();
}

// Returns true if the file path exists.
pub fn file_exists(path: &str) -> bool {
    std::fs::metadata(path).is_ok()
}

// Appends data to a file.
pub fn append_to_file(path: &str, data: &str) {
    if file_exists(path) {
        let mut file = std::fs::OpenOptions::new().append(true).open(path).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    } else {
        write_to_file(path, data);
    }
}

// Deletes the file on the named path.
pub fn delete_file(path: &str) {
    if file_exists(path) {
        std::fs::remove_file(path).unwrap();
    }
}

// Reads from a file.
pub fn read_from_file(path: &str) -> String {
    let mut f = std::fs::File::open(path).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    buffer
}

// Opens the virtual terminal.
pub fn vt_open() {
    ncurses::initscr();
    ncurses::raw();
    ncurses::scrollok(ncurses::stdscr(), true);
    ncurses::keypad(ncurses::stdscr(), true);
}

// Displays a message, then closes the virtual terminal on the next user key press.
pub fn vt_close(x: &str) {
    ncurses::addstr(x);
    ncurses::getch();
    ncurses::endwin();
}

// Hides the virtual cursor.
pub fn vt_cursor_off() {
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
}

// Displays the virtual cursor.
pub fn vt_cursor_on() {
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_VISIBLE);
}

// Hides user keypresses.
pub fn vt_keypress_off() {
    ncurses::noecho();
}

// Displays user keypresses.
pub fn vt_keypress_on() {
    ncurses::echo();
}

// Returns the number of rows in the virtual terminal.
pub fn vt_rows() -> i32 {
    let mut r: i32 = 0;
    let mut c: i32 = 0;
    ncurses::getmaxyx(ncurses::stdscr(), &mut r, &mut c);
    r
}

// Returns the number of columns in the virtual terminal.
pub fn vt_columns() -> i32 {
    let mut r: i32 = 0;
    let mut c: i32 = 0;
    ncurses::getmaxyx(ncurses::stdscr(), &mut r, &mut c);
    c
}

// Clears the virtual terminal.
pub fn vt_cls() {
    ncurses::clear();
}

// Obtains an i32 integer from a virtual terminal key press.
pub fn vt_key_i32() -> i32 {
    let ch = ncurses::getch();
    ch
}

// Obtains user input as a string with no more than x chars.
pub fn vt_input(x: i32) -> String {
    vt_keypress_on();
    let mut y = String::new();
    ncurses::getnstr(&mut y, x);
    y
}

// Displays a slice in the virtual terminal.
pub fn vt_put_slice(x: &str) {
    ncurses::addstr(x);
    ncurses::refresh();
}

// A helper function called by vt_menu.
pub fn vt_render_menu(menu: &mut Vec<String>, size: usize, count: usize) {
    vt_cls();
    vt_put_slice("\n     ");
    vt_put_slice(&mut menu[0]);
    vt_put_slice("\n\n");

    let mut n: usize = 1;
    while n < size {
        if n == count {
            vt_put_slice("   > ");
            vt_put_slice(&mut menu[n]);
            vt_put_slice("\n");
        } else {
            vt_put_slice("     ");
            vt_put_slice(&mut menu[n]);
            vt_put_slice("\n");
        }
        n += 1;
    }
}

// Returns a usize integer based on the user's selection from a menu.
pub fn vt_menu(menu: &mut Vec<String>) -> usize {
    vt_keypress_off();
    vt_cursor_off();

    let mut value: usize = 1;
    let size = menu.len();

    loop {
        vt_render_menu(menu, size, value);
        let key_press = ncurses::getch();

        if key_press == ncurses::KEY_DOWN {
            value += 1;
            if value == size {
                value = 1;
            }
            vt_render_menu(menu, size, value);
        }

        if key_press == ncurses::KEY_UP {
            value -= 1;
            if value == 0 {
                value = size - 1;
            }
            vt_render_menu(menu, size, value);
        }

        if key_press == ncurses::KEY_RIGHT {
            break;
        }

        if key_press == ncurses::KEY_ENTER {
            break;
        }

        if key_press == 10 {
            break;
        }
    }

    value
}

// A helper function called by vt_edit_prompt.
pub fn vt_render_prompt(prompt: &str, buffer: &mut String, pos: usize) {
    vt_cls();
    let mut s = String::from(prompt);
    for i in 0..pos {
        let ch = nth_char(&buffer, i);
        s.push(ch);
    }
    vt_put_slice(&s);
}

// Displays a prompt to the user with an existing buffer, which can be edited to return a new buffer.
pub fn vt_edit_prompt(prompt: &str, buffer: &mut String, max: usize) -> String {
    let mut exit = false;
    let mut result = buffer.clone();
    let mut pos = char_count(&buffer);
    let mut res = pos.clone();

    vt_render_prompt(&prompt, &mut result, pos);

    while !exit && pos < max {
        let ch = ncurses::getch();

        if ch == ncurses::KEY_LEFT && pos > 0 {
            pos -= 1;
        }
        if ch == ncurses::KEY_RIGHT && res > pos {
            pos += 1;
        }

        if ch == 127 && pos > 0 {
            pos -= 1;
            res -= 1;
            let _ = result.pop();
        } else if ch > 31 && ch < 127 {
            let ch_u8 = ch as u8;
            let ch_char = ch_u8 as char;

            if pos < res {
                result.replace_range(pos..pos + 1, &ch_char.to_string());
                pos += 1;
            } else {
                result.push(ch_char);
                pos += 1;
                res += 1;
            }
        } else if ch == 10 {
            exit = true;
        }
        vt_render_prompt(&prompt, &mut result, pos);
    }

    result
}
