use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

enum InputMode {
    Normal,
    Editing,
    EditingExisting,
}

struct App {
    todos: Vec<String>,
    done: Vec<String>,
    input: String,
    input_mode: InputMode,
    todo_list_state: ListState,
    done_list_state: ListState,
    editing_index: Option<usize>,
}

impl App {
    fn new() -> App {
        let mut todo_list_state = ListState::default();
        todo_list_state.select(Some(0));
        let mut done_list_state = ListState::default();
        done_list_state.select(Some(0));
        App {
            todos: Vec::new(),
            done: Vec::new(),
            input: String::new(),
            input_mode: InputMode::Normal,
            todo_list_state,
            done_list_state,
            editing_index: None,
        }
    }

    fn next(&mut self) {
        if self.todo_list_state.selected().is_some() {
            let i = match self.todo_list_state.selected() {
                Some(i) => {
                    if i >= self.todos.len().saturating_sub(1) {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.todo_list_state.select(Some(i));
        } else if self.done_list_state.selected().is_some() {
            let i = match self.done_list_state.selected() {
                Some(i) => {
                    if i >= self.done.len().saturating_sub(1) {
                        0
                    } else {
                           i + 1
                    }
                }                     
                None => 0,
            };
            self.done_list_state.select(Some(i));
        }
    } 

    fn previous(&mut self) {
        if self.todo_list_state.selected().is_some() {
            let i = match self.todo_list_state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.todos.len().saturating_sub(1)
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.todo_list_state.select(Some(i));
        } else if self.done_list_state.selected().is_some() {
            let i = match self.done_list_state.selected() {
                Some(i) => {
                    if i == 0  {
                        self.done.len().saturating_sub(1)
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.done_list_state.select(Some(i));
        }
    }
    fn move_to_done(&mut self) {
        if let Some(selected) = self.todo_list_state.selected() {
            if !self.todos.is_empty() {
                let item = self.todos.remove(selected);
                self.done.push(item);
                if selected > 0 && selected == self.todos.len() {
                    self.todo_list_state.select(Some(selected - 1));
                }
            }
        }
    }

    fn move_to_todo(&mut self) {
        if let Some(selected) = self.done_list_state.selected() {
            if !self.done.is_empty() {
                let item = self.done.remove(selected);
                self.todos.push(item);
                if selected > 0 && selected == self.done.len() {
                    self.done_list_state.select(Some(selected - 1));
                }
            }
        }
    }
    fn delete_selected(&mut self) {
        if let Some(selected) = self.todo_list_state.selected() {
            if !self.todos.is_empty() {
                self.todos.remove(selected);
                if selected > 0 && selected == self.todos.len() {
                    self.todo_list_state.select(Some(selected - 1));
                }
            }
        }
    }
    fn start_editing(&mut self) {
        let selected = if self.todo_list_state.selected().is_some() {
            self.todo_list_state.selected()
        } else {
            self.done_list_state.selected()
        };

        if let Some(index) = selected {
            let item = if self.todo_list_state.selected().is_some() {
                &self.todos[index]
            } else {
                &self.done[index]
            };
            self.input = item.clone();
            self.editing_index = Some(index);
            self.input_mode = InputMode::EditingExisting;
        }
    }

    fn finish_editing(&mut self) {
        if let Some(index) = self.editing_index {
            if self.todo_list_state.selected().is_some() {
                self.todos[index] = self.input.clone();
            } else if self.done_list_state.selected().is_some() {
                self.done[index] = self.input.clone()
            }
        }
        self.input.clear();
        self.editing_index = None;
        self.input_mode = InputMode::Normal;
    }
    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)?;

        writeln!(file, "[TODO]")?;
        for item in &self.todos {
            writeln!(file, "{}", item)?;
        }

        writeln!(file, "[DONE]")?;
        for item in &self.done {
            writeln!(file, "{}", item)?;
        }

        Ok(())
    }
    pub fn load_from_file(filename: &str) -> io::Result<Self> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut todos = Vec::new();
        let mut done = Vec::new();
        let mut current_list = &mut todos;

        for line in reader.lines() {
            let line = line?;
            match line.as_str() {
                "[TODO]" => current_list = &mut todos,
                "[DONE]" => current_list = &mut done,
                _ => current_list.push(line),
            }
        }
        let mut todo_list_state = ListState::default();
        if !todos.is_empty() {
            todo_list_state.select(Some(0));
        }

        let mut done_list_state = ListState::default();
        if !done.is_empty() {
            done_list_state.select(Some(0));
        }

        Ok(App {
            todos,
            done,
            input: String::new(),
            input_mode: InputMode::Normal,
            todo_list_state,
            done_list_state,
            editing_index: None,
        })
    }
}


fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    let filename = "todo_list.txt";
    let mut app = if Path::new(filename).exists() {
        App::load_from_file(filename)?
    } else {
        App::new()
    };

    let res = run_app(&mut terminal, &mut app);
    run_app(&mut terminal, &mut app)?;
    app.save_to_file(filename)?;
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: tui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('e') => app.start_editing(),
                    KeyCode::Char('i') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('j') => app.next(),
                    KeyCode::Char('k') => app.previous(),
                    KeyCode::Tab => {
                        if app.todo_list_state.selected().is_some() {
                            app.done_list_state.select(Some(0));
                            app.todo_list_state.select(None);
                        } else {
                            app.todo_list_state.select(Some(0));
                            app.done_list_state.select(None);
                        }
                    }
                    KeyCode::Enter => {
                        if app.todo_list_state.selected().is_some() {
                            app.move_to_done();
                        } else if app.done_list_state.selected().is_some() {
                            app.move_to_todo();
                        }
                    }
                    KeyCode::Char('d') => app.delete_selected(),
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        let new_todo = app.input.drain(..).collect();
                        app.todos.push(new_todo);
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
                InputMode::EditingExisting => match key.code {
                    KeyCode::Enter => app.finish_editing(),
                    KeyCode::Char(e) => {
                        app.input.push(e);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input.clear();
                        app.editing_index = None;
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                }
            }
        }
    }
}

fn ui<B: tui::backend::Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
        InputMode::EditingExisting => (
            vec![
                Span::raw("Editing existing item. Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to cancel, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to save changes"),
            ],
            Style::default().fg(Color::Yellow),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing | InputMode::EditingExisting => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);
    if let InputMode::Editing = app.input_mode {
        f.set_cursor(
            chunks[1].x + app.input.len() as u16 + 1,
            chunks[1].y + 1,
        )
    }

    let todos: Vec<ListItem> = app
        .todos
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();

        let todos = List::new(todos)
        .block(Block::default().borders(Borders::ALL).title("TODO"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

        let done: Vec<ListItem> = app
        .done
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();

    let done = List::new(done)
        .block(Block::default().borders(Borders::ALL).title("Done"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    let lists = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[2]);

    f.render_stateful_widget(todos, lists[0], &mut app.todo_list_state);
    f.render_stateful_widget(done, lists[1], &mut app.done_list_state);
}