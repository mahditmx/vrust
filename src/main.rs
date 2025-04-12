pub mod ccolor;
pub mod syntax_hl;
mod work_whit_file;
use crossterm::cursor::RestorePosition;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, event::{read, Event, KeyCode,poll}, execute, queue, style, style::{Color, Print, SetForegroundColor}, terminal};
use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use regex::Match;

struct Config{
    line_number_length: usize,
}

#[derive(Clone)]
pub struct Highlight{
    active: bool,
    language: String,
    lvl: usize,
}




fn main() {
    let mut stdout = stdout();

    let config = Config{
        line_number_length:4

    };

    let mut high_light = Highlight{
        active: true,
        language: String::from("rust"),
        lvl: 3, // 3 is Maximum highlight details.

    };
    let mut origin = config.line_number_length as u16;  // Zero start
    let mut insert = false;
    let mut command = false;
    let mut command_text = String::from(":");
    let mut lock_info_line:bool = true;
    let mut remember_cursor_move:u16 = origin;
    let mut remember_cursor_command:(u16,u16) = (origin,0);
    let mut file_path = String::from("NewFile");
    let mut is_new_file = false;
    let mut extra_show_line:u16 = 0;
    let mut show_number:bool= true;




    queue!(stdout, terminal::EnterAlternateScreen).expect("Failed to enter alternate screen");
    queue!(stdout,cursor::Show).unwrap();
    queue!(stdout, cursor::EnableBlinking).unwrap();
    queue!(stdout,cursor::MoveTo(origin, 0)).unwrap();
    terminal::enable_raw_mode().expect("Failed to enter raw mode.");
    stdout.flush().unwrap();


    let mut text:String = String::new();


    // render(text.clone(),Some(false));
    render_number_list(text.clone(),extra_show_line,show_number,&mut origin);
    // render_line_info(text.clone(),lock_info_line,file_path.clone(),is_new_file.clone(),extra_show_line);
    show_info("Use :o <file> to open or create new file.".to_string());
    stdout.flush().unwrap();

    let mut last_scroll_time = Instant::now();
    let mut last_press_time = Instant::now();
    let scroll_delay = Duration::from_millis(50);
    let mut last_key = None;
    let mut fast_scroll = false;
    loop {
        // Wait for key event
        if poll(Duration::from_millis(4)).unwrap() {
            if let Event::Key(key_event) = read().unwrap() {
                let now = Instant::now();
                let key = Some(key_event.code.clone());
                if last_key == key && now.duration_since(last_press_time) <= scroll_delay{
                    last_press_time = now;
                    fast_scroll = true;

                }else {
                    fast_scroll = false;
                }
                last_key = Some(key_event.code.clone());
                last_press_time = now;
                match key_event.code {
                    KeyCode::Esc => {
                        insert = false;
                        lock_info_line = false;
                        if remember_cursor_command != (0, 0) && command {
                            queue!(stdout, cursor::MoveTo(remember_cursor_command.0, remember_cursor_command.1)).unwrap();
                        }
                        queue!(stdout,cursor::Show).unwrap();
                        command = false;
                        render_line_info(text.clone(), lock_info_line, file_path.clone(), is_new_file.clone(), extra_show_line, origin);
                        stdout.flush().unwrap();
                    },
                    KeyCode::Char(c) => {
                        let size = terminal::size().unwrap();

                        if !insert && !command {
                            if c == ':' {
                                lock_info_line = true;
                                command = true;
                                command_text = String::from(":");
                                show_info(command_text.clone());
                                queue!(stdout,cursor::Hide).unwrap();
                                // queue!(stdout,cursor::MoveTo(0,size.1)).unwrap();
                                stdout.flush().unwrap();
                                remember_cursor_command = cursor::position().unwrap();
                            }
                            if c == 'i' {
                                insert = true;
                                command = false;
                                lock_info_line = true;
                                show_info("-- INSERT --".to_string());
                                stdout.flush().unwrap();
                                queue!(stdout,cursor::Show).unwrap();
                            }

                            continue;
                        }
                        remember_cursor_move = origin;

                        if command {
                            let cursor_pos = cursor::position().unwrap();
                            let cursor_pos = (cursor_pos.0, size.1);
                            command_text.push(c);
                            execute!(stdout, cursor::MoveTo(cursor_pos.0 +1 , cursor_pos.1)).unwrap();
                            show_info(command_text.clone());
                        } else {
                            let cursor_pos = cursor::position().unwrap();
                            let cursor_pos_new = (cursor_pos.0 - origin, cursor_pos.1);
                            let index = get_text_index(&text, cursor_pos_new.0, cursor_pos_new.1 + extra_show_line);
                            text.insert(index, c);

                            queue!(stdout, cursor::MoveTo(cursor_pos.0 +1 , cursor_pos.1)).unwrap();
                            render_line_info(text.clone(), lock_info_line, file_path.clone(), is_new_file.clone(), extra_show_line, origin);
                            render(text.clone(), None, extra_show_line, origin,high_light.clone());
                        }


                        stdout.flush().unwrap();
                    }
                    KeyCode::Enter => {
                        remember_cursor_move = origin;

                        let size = terminal::size().unwrap();
                        let cursor_pos = cursor::position().unwrap();
                        let cursor_pos_new = (cursor_pos.0 - origin, cursor_pos.1);
                        let index = get_text_index(&text, cursor_pos_new.0, cursor_pos_new.1 + extra_show_line);
                        let lines_len = text.lines().count() as u16;
                        if insert {
                            text.insert(index, '\n');
                        }


                        if command {
                            if command_text == ":!q" {
                                break;
                            }
                            if command_text == ":q" {
                                break;
                            }
                            if command_text == ":w" {
                                save(text.clone(), file_path.clone());
                                lock_info_line = false;
                                command = false;
                                queue!(stdout, cursor::MoveTo(remember_cursor_command.0, remember_cursor_command.1),cursor::Show).unwrap();
                                render_line_info(text.clone(), lock_info_line, file_path.clone(), is_new_file.clone(), extra_show_line, origin);
                                stdout.flush().unwrap();
                            }
                            if command_text == ":wq" {
                                save(text.clone(), file_path.clone());
                                break;
                            }
                            match command_text[1..].parse::<usize>(){
                                Ok(num) =>{
                                    lock_info_line = false;
                                    command = false;
                                    extra_show_line = (num - 1) as u16;
                                    queue!(stdout, cursor::MoveTo(remember_cursor_command.0, remember_cursor_command.1),cursor::Show).unwrap();
                                    render_number_list(text.clone(), extra_show_line, show_number, &mut origin);
                                    render(text.clone(), None, extra_show_line, origin,high_light.clone());
                                    render_line_info(text.clone(), lock_info_line, file_path.clone(), is_new_file.clone(), extra_show_line, origin);
                                    stdout.flush().unwrap();
                                },
                                Err(_) => {}
                            }



                            let cmd = command_text.split(' ').collect::<Vec<&str>>();

                            if cmd[0] == ":set"{
                                let command_text = cmd[1..].join(" ");
                                if command_text.trim() == "number" { // todo: Fix crash after turn on.
                                    if show_number {
                                        show_number = false;
                                        origin = 0;
                                    } else {
                                        show_number = true;
                                        origin = 4;
                                    }
                                    lock_info_line = false;
                                    command = false;

                                    queue!(stdout, cursor::MoveTo(remember_cursor_command.0, remember_cursor_command.1),cursor::Show).unwrap();
                                    render_number_list(text.clone(), extra_show_line, show_number, &mut origin);
                                    render(text.clone(), None, extra_show_line, origin,high_light.clone());
                                    render_line_info(text.clone(), lock_info_line, file_path.clone(), is_new_file.clone(), extra_show_line, origin);

                                    stdout.flush().unwrap();



                                }
                                if command_text.trim() == "hl" || command_text.trim() == "highlight" {
                                    if high_light.active{
                                        high_light = Highlight{
                                            active: false,
                                            ..high_light
                                        }
                                    } else {
                                        high_light = Highlight{
                                            active: true,
                                            ..high_light
                                        }
                                    }
                                    lock_info_line = false;
                                    command = false;

                                    queue!(stdout, cursor::MoveTo(remember_cursor_command.0, remember_cursor_command.1),cursor::Show).unwrap();
                                    render_number_list(text.clone(), extra_show_line, show_number, &mut origin);
                                    render(text.clone(), None, extra_show_line, origin,high_light.clone());
                                    render_line_info(text.clone(), lock_info_line, file_path.clone(), is_new_file.clone(), extra_show_line, origin);

                                    stdout.flush().unwrap();


                                }
                            }





                            // let cmd = command_text.clone();
                            if cmd[0] == ":o" {
                                lock_info_line = false;
                                command = false;

                                let cmd = cmd[1].to_string();
                                show_info(format!("Opening \"{}\" file... Big one?", cmd));
                                stdout.flush().unwrap();


                                queue!(stdout,
                                    Clear(ClearType::All),
                                    cursor::MoveTo(origin, 0),
                                    cursor::Show,
                                ).unwrap();

                                file_path = cmd.clone();
                                let rx: (String, bool) = handle_open(cmd.clone(), origin);
                                extra_show_line = 0;
                                text = rx.0;
                                is_new_file = rx.1;

                                render(text.clone(), None, extra_show_line, origin,high_light.clone());
                                render_number_list(text.clone(), extra_show_line, show_number, &mut origin);
                                render_line_info(text.clone(), lock_info_line, file_path.clone(), is_new_file.clone(), extra_show_line, origin);
                                stdout.flush().unwrap();
                            }

                            continue;
                        }


                        if (size.1 - cursor_pos.1) < 8 && extra_show_line <= lines_len {
                            extra_show_line += 1;
                            render(text.clone(), None, extra_show_line, origin,high_light.clone());

                            render_number_list(text.clone(), extra_show_line, show_number, &mut origin);
                            render_line_info(text.clone(), lock_info_line, file_path.clone(), is_new_file.clone(), extra_show_line, origin);
                            execute!(stdout, cursor::MoveTo(origin, cursor_pos.1)).unwrap();

                            stdout.flush().unwrap();
                            continue;
                        }

                        let new_y = cursor_pos.1 + 1;

                        render(text.clone(), None, extra_show_line, origin,high_light.clone());
                        render_line_info(text.clone(), lock_info_line, file_path.clone(), is_new_file.clone(), extra_show_line, origin);

                        execute!(stdout, cursor::MoveTo(origin, new_y)).unwrap();


                        stdout.flush().unwrap();
                    }
                    KeyCode::Backspace => {
                        remember_cursor_move = origin;
                        let cursor_pos = cursor::position().unwrap();
                        let cursor_pos_new = (cursor_pos.0 - origin, cursor_pos.1);
                        let mut go_previous_line = false;

                        if cursor_pos_new.0 == 0 {
                            if cursor_pos_new.1 == 0 {
                                continue;
                            }
                            go_previous_line = true;
                        }


                        let index = get_text_index(&text, cursor_pos_new.0, cursor_pos_new.1 + extra_show_line);


                        if go_previous_line && !command {
                            let lines = text.split('\n').collect::<Vec<&str>>();
                            if let Some(line) = lines.get((cursor_pos.1 + extra_show_line - 1) as usize) {
                                let line_len = line.len() as u16;
                                execute!(stdout, cursor::MoveTo(origin + line_len , cursor_pos.1 -1)).unwrap();
                            } else {
                                execute!(stdout, cursor::MoveTo(origin, cursor_pos.1 -1)).unwrap();
                            }

                            // render_number_list(text.clone());
                            text.remove(index - 1);
                            render(text.clone(), None, extra_show_line, origin,high_light.clone());
                        } else {
                            execute!(stdout, cursor::MoveTo(cursor_pos.0 - 1 , cursor_pos.1)).unwrap();
                            // let index = get_text_index(&text, cursor_pos_new.0, cursor_pos_new.1 + extra_show_line);
                            if insert {
                                text.remove(index - 1);
                                render(text.clone(), None, extra_show_line, origin,high_light.clone());
                            }
                            if command {
                                command_text.pop();
                                show_info(command_text.clone());
                            }
                        }

                        render_number_list(text.clone(), extra_show_line, show_number, &mut origin);

                        render_line_info(text.clone(), lock_info_line, file_path.clone(), is_new_file.clone(), extra_show_line, origin);

                        stdout.flush().unwrap();
                    }
                    KeyCode::Left => {
                        remember_cursor_move = origin;
                        let cursor_pos = cursor::position().unwrap();

                        if cursor_pos.0 == origin || command {
                            continue;
                        }
                        queue!(stdout, cursor::MoveTo(cursor_pos.0 -1, cursor_pos.1)).unwrap();
                        render_line_info(text.clone(), lock_info_line, file_path.clone(), is_new_file.clone(), extra_show_line, origin);

                        stdout.flush().unwrap();
                    }
                    KeyCode::Right => {
                        remember_cursor_move = origin;
                        let cursor_pos = cursor::position().unwrap();

                        let size = terminal::size().unwrap();


                        let working_line;
                        if let Some(line) = text.lines().collect::<Vec<&str>>().get((cursor_pos.1 + extra_show_line) as usize) {
                            working_line = *line;
                        } else {
                            continue;
                        }


                        let working_line_len = working_line.len() as u16;
                        if cursor_pos.0 == working_line_len + origin || working_line_len == origin || cursor_pos.0 == (size.0 - origin) - 1 || command {
                            continue;
                        }


                        queue!(stdout, cursor::MoveTo(cursor_pos.0 +1, cursor_pos.1)).unwrap();
                        render_line_info(text.clone(), lock_info_line, file_path.clone(), is_new_file.clone(), extra_show_line, origin);

                        stdout.flush().unwrap();
                    }
                    KeyCode::Up => {

                        if ! (now.duration_since(last_scroll_time) >= scroll_delay) {
                            continue;
                        }

                        last_scroll_time = now;


                        let cursor_pos = cursor::position().unwrap();

                        if cursor_pos.1 == 0 || command {
                            continue;
                        }
                        if cursor_pos.1 == 1  {
                            fast_scroll = false;
                        }

                        if cursor_pos.1 < 8 && extra_show_line != 0 {
                            if fast_scroll{
                                extra_show_line -= 2;

                            }else{
                                extra_show_line -= 1;

                            }
                            render(text.clone(), None, extra_show_line, origin,high_light.clone());
                            render_number_list(text.clone(), extra_show_line, show_number, &mut origin);
                            render_line_info(text.clone(), lock_info_line, file_path.clone(), is_new_file.clone(), extra_show_line, origin);

                            stdout.flush().unwrap();

                            continue;
                        }

                        let lines = text.split('\n').collect::<Vec<&str>>();
                        if let Some(line) = lines.get((cursor_pos.1 - 1 + extra_show_line) as usize) {
                            let line_len = line.len() as u16 + origin + 1;

                            if line_len >= cursor_pos.0 {
                                let mut cursor_pos_0 = cursor_pos.0;

                                if cursor_pos.0 == origin {
                                    cursor_pos_0 = remember_cursor_move;
                                    if line_len <= cursor_pos_0 {
                                        cursor_pos_0 = origin;
                                    }
                                }

                                if fast_scroll{
                                    queue!(stdout, cursor::MoveTo(cursor_pos_0, cursor_pos.1- 2)).unwrap();

                                }else{
                                    queue!(stdout, cursor::MoveTo(cursor_pos_0, cursor_pos.1- 1)).unwrap();

                                }                            } else {
                                if cursor_pos.0 != origin {
                                    remember_cursor_move = cursor_pos.0;
                                }
                                if fast_scroll{
                                    queue!(stdout, cursor::MoveTo(origin, cursor_pos.1- 2)).unwrap();

                                }else{
                                    queue!(stdout, cursor::MoveTo(origin, cursor_pos.1- 1)).unwrap();

                                }                            }
                        } else {
                            if cursor_pos.0 != origin {
                                remember_cursor_move = cursor_pos.0;
                            }
                            if fast_scroll{
                                queue!(stdout, cursor::MoveTo(origin, cursor_pos.1- 2)).unwrap();

                            }else{
                                queue!(stdout, cursor::MoveTo(origin, cursor_pos.1- 1)).unwrap();

                            }                        }


                        render_line_info(text.clone(), lock_info_line, file_path.clone(), is_new_file.clone(), extra_show_line, origin);

                        stdout.flush().unwrap();
                    }
                    KeyCode::Down => {

                        if ! (now.duration_since(last_scroll_time) >= scroll_delay) {
                            continue;
                        }

                        last_scroll_time = now;

                        let cursor_pos = cursor::position().unwrap();
                        let size = terminal::size().unwrap();


                        let lines = text.split('\n').collect::<Vec<&str>>(); // todo : only Split visible.
                        let lines_len = lines.len() as u16;

                        if cursor_pos.1 == size.1 - 3 || lines_len <= cursor_pos.1 + 2  {
                            fast_scroll = false;
                        }

                        if cursor_pos.1 == size.1 - 2 || lines_len <= cursor_pos.1 + 1 || command {
                            continue;
                        }

                        if (size.1 - cursor_pos.1) < 8 && extra_show_line + size.1 + 1 <= lines_len {
                            if fast_scroll{
                                extra_show_line += 2;

                            }else{
                                extra_show_line += 1;

                            }

                            render(text.clone(), None, extra_show_line, origin,high_light.clone());
                            render_line_info(text.clone(), lock_info_line, file_path.clone(), is_new_file.clone(), extra_show_line, origin);
                            render_number_list(text.clone(), extra_show_line, show_number, &mut origin);
                            stdout.flush().unwrap();
                            continue;
                        }


                        if let Some(line) = lines.get((cursor_pos.1 + 1 + extra_show_line) as usize) {
                            let line_len = line.len() as u16 + origin + 1;
                            if line_len >= cursor_pos.0 {
                                let mut cursor_pos_0 = cursor_pos.0;

                                if cursor_pos.0 == origin {
                                    cursor_pos_0 = remember_cursor_move;
                                    if line_len <= cursor_pos_0 {
                                        cursor_pos_0 = origin;
                                    }
                                }
                                if fast_scroll{
                                    queue!(stdout, cursor::MoveTo(cursor_pos_0, cursor_pos.1+ 2)).unwrap();

                                }else{
                                    queue!(stdout, cursor::MoveTo(cursor_pos_0, cursor_pos.1+ 1)).unwrap();

                                }
                            } else {
                                if cursor_pos.0 != origin {
                                    remember_cursor_move = cursor_pos.0;
                                }
                                if fast_scroll{
                                    queue!(stdout, cursor::MoveTo(origin, cursor_pos.1+ 2)).unwrap();

                                }else{
                                    queue!(stdout, cursor::MoveTo(origin, cursor_pos.1+ 1)).unwrap();

                                }
                            }
                        } else {
                            if cursor_pos.0 != origin {
                                remember_cursor_move = cursor_pos.0;
                            }
                            if fast_scroll{
                                queue!(stdout, cursor::MoveTo(origin, cursor_pos.1+ 2)).unwrap();

                            }else{
                                queue!(stdout, cursor::MoveTo(origin, cursor_pos.1+ 1)).unwrap();

                            }
                        }

                        render_number_list(text.clone(), extra_show_line, show_number, &mut origin);
                        render_line_info(text.clone(), lock_info_line, file_path.clone(), is_new_file.clone(), extra_show_line,origin);

                        stdout.flush().unwrap();
                    }
                    KeyCode::Tab => {
                        if !insert{
                            continue;
                        }

                        let cursor_pos = cursor::position().unwrap();
                        let cursor_pos_new = (cursor_pos.0 - origin, cursor_pos.1);
                        let index = get_text_index(&text, cursor_pos_new.0, cursor_pos_new.1 + extra_show_line);

                        let req_space = 4 - (cursor_pos_new.0 % 4) as usize;
                        let spaces = " ".repeat(req_space);   // Repeat space character

                        
                        text.insert_str(index,&spaces); //todo: When remove all the Tab must removed not only one space
                        queue!(stdout, cursor::MoveTo(cursor_pos.0 +req_space as u16 , cursor_pos.1)).unwrap();

                        render(text.clone(), None, extra_show_line, origin, high_light.clone());
                        render_line_info(text.clone(), lock_info_line, file_path.clone(), is_new_file.clone(), extra_show_line, origin);
                        stdout.flush().unwrap();

                       
                        

                        
                       



                    }
                    _ => {}
                }
            }
        }else{

        }
    }


    execute!(stdout, terminal::LeaveAlternateScreen).expect("Failed to leave alternate screen");
    execute!(stdout,cursor::Show).unwrap();
    terminal::disable_raw_mode().expect("Failed to enter raw mode.");
}


fn render(text: String, restore: Option<bool>, extra_show_line: u16, origin: u16, highlight: Highlight) {
    let restore = restore.unwrap_or(true);
    let mut stdout = stdout();
    let line_count_org = text.split('\n').count();
    let mut text = text;

    let line_count = text.split('\n').count();
    let extra_show_line = extra_show_line as usize;

    let lines = text.clone();
    let lines = lines.lines().collect::<Vec<&str>>();

    let size = terminal::size().unwrap();

    let mut visible_part = String::new();


    for i in 0..size.1 as usize {
        let line = i + extra_show_line ;
        if line >= lines.iter().count(){
            break;
        }
        let to_print = lines.get(line).unwrap().to_string();
        visible_part.push_str(format!("{}\n", to_print).as_str());

    }

    let mut text = visible_part;

    if highlight.active{
        text = syntax_hl::render(text.clone(),highlight);
    }

    let lines = text.lines().collect::<Vec<&str>>();
    for i in 0..size.1 as usize {
        let line = i  ;
        if line >= lines.iter().count(){
            break;
        }
        let mut to_print = lines.get(line).unwrap().to_string();
        if line_count != line_count_org{
            to_print = "Something went wrong in Highlight.rs please turn it off using :set hl".to_string();
        }
        queue!(stdout,
            cursor::SavePosition,
            cursor::MoveTo(origin,i as u16),
            Clear(ClearType::UntilNewLine),
            Print(to_print),
        ).unwrap();
        if restore{
            queue!(stdout,
                RestorePosition
            ).unwrap()
        }

    }
}

fn render_line_info(text:String,lock:bool,file_path:String,is_new_file:bool,extra_show_line:u16,org_origin:u16) {

    if lock{
        return;
    }
    let mut stdout = stdout();

    let origin = 0;
    let cursor_pos = cursor::position().unwrap();
    let size = terminal::size().unwrap();

    let lines_len = text.lines().count();

    let info_line_1:String;
    if is_new_file{
        info_line_1 = format!("\"{}\" [New], {}L",file_path,lines_len,);

    }else{
        info_line_1 = format!("\"{}\", {}L",file_path,lines_len,);
    }
    let percentage_of_top = ((extra_show_line as f32 / (lines_len as u16 - cursor_pos.1) as f32) *100.0 ) as u16 ;

    let mut info_line_2 = String::new();
    if percentage_of_top == 0{
        info_line_2 = format!("{}:{}, {}x{}, TOP",cursor_pos.0 - org_origin +1, cursor_pos.1 +1, size.0, size.1);

    }else if percentage_of_top >= 98 { // Better syntax TODO
        info_line_2 = format!("{}:{}, {}x{}, BOT",cursor_pos.0 - org_origin +1 , cursor_pos.1 + extra_show_line +1, size.0, size.1);

    }

    else{
        info_line_2 = format!("{}:{}, {}x{}, {}%",cursor_pos.0 - org_origin + 1, cursor_pos.1 + extra_show_line +1, size.0, size.1,percentage_of_top);

    }




    let space_req = size.0 - origin - info_line_1.len() as u16 - info_line_2.len() as u16 -1 ;
    let space = " ".repeat(space_req as usize);

    let info_line = format!("{}{}{}",info_line_1,space,info_line_2);

    queue!(stdout,
        cursor::SavePosition,
        cursor::MoveTo(origin, size.1),
        terminal::Clear(ClearType::CurrentLine),
        Print(info_line),
        RestorePosition
    ).unwrap();
}
fn show_info(text:String){
    let mut stdout = stdout();

    let origin = 1;
    let size = terminal::size().unwrap();
    let info_line = format!("{}",text);

    queue!(stdout,
        cursor::SavePosition,
        style::SetAttribute(style::Attribute::Bold),
        cursor::MoveTo(origin, size.1),
        terminal::Clear(ClearType::CurrentLine),
        Print(info_line),
        RestorePosition
    ).unwrap();
}


fn render_number_list(text: String, extra_show_line: u16, show_number: bool, origin:&mut u16) {
    let mut stdout = stdout();
    let size = terminal::size().unwrap();

    let lines_len = text.lines().count() as u16;

    for visual_row in 0..(size.1 - 1) {
        let line_number = visual_row + extra_show_line + 1;

        if !show_number {
            continue;
        }
        let mut number_str = " ".to_string();

        if show_number{
            number_str = (visual_row+extra_show_line+1) .to_string();
        }

        let mut extra_space = " ".repeat(origin.saturating_sub((number_str.len() + 1) as u16) as usize);
        // if true == false{
        //     extra_space = " ".to_string();
        //     *origin += 1;
        //     execute!(stdout,Clear(ClearType::All)).unwrap();
        //     render(text.clone(),None,extra_show_line,origin.clone(),high_light.clone());
        //
        // }


        let string = if line_number >= lines_len + 2 {
            format!("{}{} {}~{}", extra_space, line_number, ccolor::colors::BLUE, ccolor::colors::RESET)
        } else {
            format!("{}{}", extra_space, line_number)
        };




        if lines_len < visual_row {
            queue!(
            stdout,
            cursor::SavePosition,
            cursor::MoveTo(0, visual_row),
            SetForegroundColor(Color::AnsiValue(238)),
            terminal::Clear(ClearType::CurrentLine),
            Print(string),
            RestorePosition
        ).unwrap();
        }else {
            queue!(
            stdout,
            cursor::SavePosition,
            cursor::MoveTo(0, visual_row),
            SetForegroundColor(Color::AnsiValue(238)),
            Print(string),
            RestorePosition
        ).unwrap();
        }
    }
}



fn get_text_index(text: &str, x: u16, y: u16) -> usize {
    let lines: Vec<&str> = text.lines().collect();
    let mut index = 0;

    for i in 0..y as usize {
        if i < lines.len() {
            index += lines[i].len() + 1; // +1 for '\n'
        }
    }

    index + x as usize
}

fn save(text:String, path:String){

    work_whit_file::write_file(path,text);


}

fn handle_open(cmd:String,origin:u16,) -> (String,bool){
    let mut stdout = stdout();
    let size = terminal::size().unwrap();
    if work_whit_file::file_exists(cmd.clone()){
        let text = work_whit_file::read_file(cmd.clone());
        let lines_len = text.lines().count() as u16;
        if lines_len > size.1 - 2{
            queue!(stdout,
            cursor::MoveTo(origin, 0),
            ).unwrap();
        }else{
            queue!(stdout,
            cursor::MoveTo(origin, lines_len),
            ).unwrap();
        }



        (text,false)

    }
    else{
        queue!(stdout,
            cursor::MoveTo(origin, 0),
        ).unwrap();
        (String::new(),true) // true mean NEW FILE

    }


}



