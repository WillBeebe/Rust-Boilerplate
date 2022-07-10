#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_macros)]

extern crate ncurses;

use ncurses::*;
use ncurses::CURSOR_VISIBILITY::*;

use std::path::*;
use std::process::*;

use std::fs::*;
use std::io::*;

use std::time::*;
use std::*;

use rand::rngs::*;
use rand::*;

// Prompts the user for input and returns it as a string.
pub fn input(prompt:&str) -> String {
    print!("\n{}\n> ", prompt);
    stdout().flush().unwrap();
    let mut answer = String::new();
    stdin().read_line(&mut answer).unwrap();
    return answer;
}

// Clears the terminal window on SOME terminal emulators.
pub fn cls() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

// Executes slices as unix shell commands.
pub fn unix_shell(x:&str) {
    let mut commands = x.trim().split(" | ").peekable();
    let mut previous_command = None;

    while let Some(command) = commands.next() {
        let mut parts = command.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }

                previous_command = None;
            }
            "exit" => return,
            command => {
                let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                    Stdio::from(output.stdout.unwrap())
                });

                let stdout = if commands.peek().is_some() {
                    Stdio::piped()
                } else {
                    Stdio::inherit()
                };

                let output = Command::new(command)
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
    let d = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let _rng = StdRng::seed_from_u64(d.as_secs());
}

// Generates a pseudo-random number between x and y.
pub fn pseudo(x:i32, y:i32) -> i32 {
    return thread_rng().gen_range(x..y + 1);
}

// A macro to initialise a vector of strings.
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

// Returns the nth char (zero indexed) from a slice.
pub fn nth_char(x:&str, n:usize) -> char {
    return x.chars().nth(n).unwrap();
}

// Checks if a slice consists only of digits.
pub fn is_digits(x:&str) -> bool {
    return x.chars().all(char::is_numeric);
}

// Checks if a slice represents a positive integer.
pub fn is_pos_int(x:&str) -> bool {
    if !is_digits(x) { return false; }
    if nth_char(x, 0) == '0' { return false; }
    return true;
}

// Checks if a slice represents a negative integer.
pub fn is_neg_int(x:&str) -> bool {
    if nth_char(x, 0) != '-' { return false; }
    if nth_char(x, 1) == '0' { return false; }
    if !nth_char(x, 1).is_ascii_digit() { return false; }

    let mut y = x.to_string();
    let _ = &mut y.remove(0);
    let _ = &mut y.remove(0);

    let z = y.as_str();
    if !is_digits(z) { return false; }
    return true;
}

// Checks if a slice represents an integer.
pub fn is_int(x:&str) -> bool {
    return is_pos_int(x) || x == "0" || is_neg_int(x);
}

// Checks if a slice represents a float.
pub fn is_float(x:&str) -> bool {
    return x.parse::<f64>().is_ok();
}

// Converts a slice to an i32 integer.
pub fn to_int(x:&str) -> i32 {
    return x.parse::<i32>().unwrap();
}

// Converts a slice to an f64 float.
pub fn to_float(x:&str) -> f64 {
    return x.parse::<f64>().unwrap();
}

// Counts the number of chars in a slice.
pub fn char_count(x:&str) -> usize {
    return x.chars().count();
}

// Counts the number of words in a slice.
pub fn word_count(x:&str) -> usize {
    if x == "" {
        return 0;
    }
    let mut count: usize = 0;
    let word_vec = x.split(" ");
    for _ in word_vec {
        count += 1;
    }
    return count;
}

// Counts the number of lines in a slice.
pub fn line_count(x:&str) -> usize {
    if x == "" {
        return 0;
    }
    let mut count: usize = 0;
    let word_vec = x.split("\n");
    for _ in word_vec {
        count += 1;
    }
    return count;
}

// Returns the nth word (zero indexed) from a slice.
pub fn nth_word(x:&str, y:usize) -> &str {
    if x == "" || y >= word_count(x) {
        return "";
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
    return result;
}

// Returns the nth line (zero indexed) from a slice.
pub fn nth_line(x:&str, y:usize) -> &str {
    if x == "" || y >= line_count(x) {
        return "";
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
    return result;
}

// Removes the nth word (zero indexed) from a slice and returns it as a string.
pub fn remove_nth_word(x:&str, y:usize) -> String {
    if x == "" || y >= word_count(x) {
        return x.to_string();
    }

    let mut result = String::new();
    let word_vec = x.split(" ");
    let mut count: usize = 0;

    for r in word_vec {
        if y != count {
            result.push_str(&r);
            result.push(' ');
        }
        count += 1;
    }

    let _ = result.pop();
    return result;
}

// Removes the nth line (zero indexed) from a slice and returns it as a string.
pub fn remove_nth_line(x:&str, y:usize) -> String {
    if x == "" || y >= line_count(x) {
        return x.to_string();
    }

    let mut result = String::new();
    let line_vec = x.split("\n");
    let mut count: usize = 0;

    for r in line_vec {
        if y != count {
            result.push_str(&r);
            result.push('\n');
        }
        count += 1;
    }

    let _ = result.pop();
    return result;
}

// Inserts a word at the nth position (zero indexed) of a slice and returns that as a string.
pub fn insert_word_at(x:&str, y:&str, z:usize) -> String {
    if x == "" || y == "" || z >= word_count(x) {
        return x.to_string();
    }

    let mut result = String::new();
    let word_vec = x.split(" ");
    let mut count: usize = 0;

    for r in word_vec {
        if z == count {
            result.push_str(&y);
            result.push(' ');
        }
        result.push_str(&r);
        result.push(' ');
        count += 1;
    }

    let _ = result.pop();
    return result;
}

// Inserts a line at the nth position (zero indexed) of a slice and returns that as a string.
pub fn insert_line_at(x:&str, y:&str, z:usize) -> String {
    if x == "" || y == "" || z >= line_count(x) {
        return x.to_string();
    }

    let mut result = String::new();
    let line_vec = x.split("\n");
    let mut count: usize = 0;

    for r in line_vec {
        if z == count {
            result.push_str(&y);
            result.push('\n');
        }
        result.push_str(&r);
        result.push('\n');
        count += 1;
    }

    let _ = result.pop();
    return result;
}

// Replaces a word at the nth position (zero indexed) of a slice and returns that as a string.
pub fn replace_word_at(x:&str, y:&str, z:usize) -> String {
    if x == "" || y == "" || z >= word_count(x) {
        return x.to_string();
    }

    let mut result = String::new();
    let word_vec = x.split(" ");
    let mut count: usize = 0;

    for r in word_vec {
        if z == count {
            result.push_str(&y);
            result.push(' ');
        } else {
            result.push_str(&r);
            result.push(' ');
        }
        count += 1;
    }

    let _ = result.pop();
    return result;
}

// Replaces a line at the nth position (zero indexed) of a slice and returns that as a string.
pub fn replace_line_at(x:&str, y:&str, z:usize) -> String {
    if x == "" || y == "" || z >= line_count(x) {
        return x.to_string();
    }

    let mut result = String::new();
    let line_vec = x.split("\n");
    let mut count: usize = 0;

    for r in line_vec {
        if z == count {
            result.push_str(&y);
            result.push('\n');
        } else {
            result.push_str(&r);
            result.push('\n');
        }
        count += 1;
    }

    let _ = result.pop();
    return result;
}

// Writes data to a file.
pub fn write_to_file(path:&str, data:&str) {
    fs::write(path, data).unwrap();
}

// Returns true if the file path exists.
pub fn file_exists(path:&str) -> bool {
    return metadata(path).is_ok();
}

// Appends data to a file.
pub fn append_to_file(path:&str, data:&str) {
    if file_exists(path) {
        let mut file = OpenOptions::new().append(true).open(path).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    } else {
        write_to_file(path, data);
    }
}

// Deletes the file on the named path.
pub fn delete_file(path:&str) {
    if file_exists(path) {
        remove_file(path).unwrap();
    }
}

// Reads data from a file into an vector of strings.
pub fn read_to_vector(path:&str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if file_exists(path) {
        let data = fs::read_to_string(path).unwrap();

        let lines = data.split("\n");
        for l in lines {
            result.push(l.to_string());
        }
    }
    return result;
}

// Appends data from a vector of strings to a file.
pub fn append_from_vector(v:Vec<String>, path:&str) {
    for line in v {
        append_to_file(path, line.as_str());
        append_to_file(path, "\n");
    }
}

// Opens the virtual terminal.
pub fn vt_open() {
    initscr();
    raw();
    scrollok(stdscr(), true);
    keypad(stdscr(), true);
}

// Displays a message, then closes the virtual terminal on the next user key press.
pub fn vt_close(x:&str) {
    addstr(x);
    getch();
    endwin();
}

// Hides the virtual cursor.
pub fn vt_cursor_off() { curs_set(CURSOR_INVISIBLE); }

// Displays the virtual cursor.
pub fn vt_cursor_on() { curs_set(CURSOR_VISIBLE); }

// Hides user keypresses.
pub fn vt_keypress_off() { noecho(); }

// Displays user keypresses.
pub fn vt_keypress_on() { echo(); }

// Returns the number of rows in the virtual terminal.
pub fn vt_rows() -> i32 {
    let mut r:i32 = 0;
    let mut c:i32 = 0;
    getmaxyx(stdscr(), &mut r, &mut c);
    return r;
}

// Returns the number of columns in the virtual terminal.
pub fn vt_columns() -> i32 {
    let mut r:i32 = 0;
    let mut c:i32 = 0;
    getmaxyx(stdscr(), &mut r, &mut c);
    return c;
}

// Clears the virtual terminal.
pub fn vt_cls() { clear(); }

// Obtains user input as a string with no more than x chars.
pub fn vt_input(x: i32) -> String {
    let mut y = String::new();
    getnstr(&mut y, x);
    return y;
}

// Obtains an i32 integer from a virtual terminal key press.
pub fn vt_key_i32() -> i32 {
    let ch = getch();
    return ch;
}

// Obtains a u8 from a virtual terminal key press.
pub fn vt_key_u8() -> u8 {
    let ch = getch();
    let ch_u8 = ch as u8;
    return ch_u8;
}

// Obtains a char from a virtual terminal key press.
pub fn vt_key_char() -> char {
    let ch = getch();
    let ch_u8 = ch as u8;
    let ch_char = ch_u8 as char;
    return ch_char;
}

// Displays a slice in the virtual terminal.
pub fn vt_put_slice(x:&str) {
    addstr(x);
    refresh();
}

// A helper function called by vt_menu.
pub fn vt_render_menu(menu:&mut Vec<String>, size:usize, count:usize) {
    vt_cls();
    vt_put_slice("\n     ");
    vt_put_slice(&mut menu[0]);
    vt_put_slice("\n\n");

    let mut n:usize = 1;
    while n < size {
        if n == count {
            vt_put_slice("   > ");
            vt_put_slice(&mut menu[n]);
            vt_put_slice("\n");
        }
        else {
            vt_put_slice("     ");
            vt_put_slice(&mut menu[n]);
            vt_put_slice("\n");
        }
        n += 1;
    }
}

// Returns a usize integer based on the user's selection from a menu.
pub fn vt_menu(menu:&mut Vec<String>) -> usize {
    vt_keypress_off();
    vt_cursor_off();

    let mut value:usize = 1;
    let size = menu.len();

    loop {
        vt_render_menu(menu, size, value);
        let key_press = getch();

        if key_press == KEY_DOWN {
            value += 1;
            if value == size {
                value = 1;
            }
            vt_render_menu(menu, size, value);
        }

        if key_press == KEY_UP {
            value -= 1;
            if value == 0 {
                value = size - 1;
            }
            vt_render_menu(menu, size, value);
        }

        if key_press == KEY_RIGHT {
            break;
        }

        if key_press == KEY_ENTER {
            break;
        }

        if key_press == 10 {
            break;
        }
    }

    return value;
}

// A helper function called by vt_edit_prompt.
pub fn vt_render_prompt(prompt:&str, buffer:&mut String, pos:usize) {
    vt_cls();
    let mut s = String::from(prompt);
    for i in 0..pos { 
        let ch = nth_char(&buffer, i);
        s.push(ch); 
    }
    vt_put_slice(&s);
}

// Displays a prompt to the user with an existing buffer, which can be edited to return a new buffer.
pub fn vt_edit_prompt(prompt:&str, buffer:&mut String, max:usize) -> String {
    let mut exit = false;
    let mut result = buffer.clone();
    let mut pos = char_count(&buffer);
    let mut res = pos.clone();

    vt_render_prompt(&prompt, &mut result, pos);

    while !exit && pos < max {
        let ch = getch();

        if ch == KEY_LEFT && pos > 0 { pos -= 1; }
        if ch == KEY_RIGHT && res > pos { pos += 1; }
        
        if ch == 127 && pos > 0 {
            pos -= 1;
            res -= 1;
            let _ = result.pop();
        }

        else if ch > 31 && ch < 127 {
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
        }

        else if ch == 10 { exit = true; }
        vt_render_prompt(&prompt, &mut result, pos);
    }

    return result;
}
