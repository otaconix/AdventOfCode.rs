use aoc_timing::trace::log_run;
use intcode::{Computer, OpCode, SplitIO};
use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
    io::{self},
};

type Input = Computer;
type Output1 = usize;
type Output2 = i64;

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl Tile {
    fn from_id(id: i64) -> Self {
        match id {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            _ => panic!("Invalid tile ID: {id}"),
        }
    }

    #[cfg(feature = "tui")]
    fn to_char(&self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::Wall => '█',
            Tile::Block => '▢',
            Tile::HorizontalPaddle => '▬',
            Tile::Ball => '●',
        }
    }
}

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(mut input: I) -> Input {
    let line = input.next().unwrap();

    Computer::parse(line.as_ref())
}

fn part_1(input: &Input) -> Output1 {
    let mut computer = input.clone();
    let mut io = VecDeque::new();

    assert_eq!(computer.run(&mut io), OpCode::Terminate);
    assert!(io.len().is_multiple_of(3));

    println!("IO length: {}", io.len());

    let display: HashMap<_, _> = io
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|mut chunk| {
            (
                (chunk.next().unwrap(), chunk.next().unwrap()),
                chunk.next().unwrap(),
            )
        })
        .collect();

    display
        .into_iter()
        .filter(|(_coord, tile_id)| *tile_id == 2)
        .count()
}

#[cfg(feature = "tui")]
mod tui {
    use std::{
        thread::sleep,
        time::{Duration, Instant},
    };

    use super::*;
    use grid::Grid;
    use ratatui::{
        DefaultTerminal, Frame,
        layout::{Alignment, Flex, Layout, Rect},
        style::{Modifier, Style},
        text::Text,
        widgets::{Block, BorderType, Padding, Paragraph},
    };

    pub struct TuiState {
        terminal: DefaultTerminal,
        display: Grid<Tile>,
        start_time: Instant,
        should_render: bool,
        delay: Option<u64>,
    }

    fn create_centered_layout(frame: &Frame) -> Rect {
        let area = frame.area();
        let horizontal = Layout::horizontal([42]).flex(Flex::Center);
        let vertical = Layout::vertical([26]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);

        area
    }

    impl TuiState {
        pub fn init() -> Self {
            let terminal = ratatui::init();
            let display: Grid<Tile> =
                std::iter::repeat_n(std::iter::repeat_n(Tile::Empty, 40).collect_vec(), 30)
                    .collect();
            let delay = std::env::var("DELAY_MILLIS")
                .map(|delay| delay.parse().expect("Invalid DELAY_MILLIS value"))
                .ok();

            Self {
                terminal,
                display,
                start_time: Instant::now(),
                should_render: false,
                delay,
            }
        }

        pub fn update(&mut self, score: i64, x: i64, y: i64, tile: &Tile) {
            self.display.update(x as usize, y as usize, *tile);

            if self.should_render && tile != &Tile::Empty {
                let display_string = (0..=self.display.height())
                    .map(|row| {
                        self.display
                            .row(row)
                            .map(|tile| tile.to_char())
                            .collect::<String>()
                    })
                    .join("\n");

                self.terminal
                    .draw(|frame| {
                        let run_duration = Instant::now().duration_since(self.start_time);
                        let display_text = Paragraph::new(Text::raw(display_string)).block(
                            Block::bordered()
                                .border_type(BorderType::Rounded)
                                .padding(Padding::symmetric(1, 2))
                                .title(format!(" [ Score: {score} ] "))
                                .title_bottom(format!(
                                    " [ {:02}:{:02} ] ",
                                    run_duration.as_secs() / 60,
                                    run_duration.as_secs() % 60
                                ))
                                .title_alignment(Alignment::Center),
                        );

                        frame.render_widget(display_text, create_centered_layout(frame));
                    })
                    .unwrap();

                if let Some(delay) = self.delay {
                    sleep(Duration::from_millis(delay));
                }
            }
        }

        pub fn start_rendering(&mut self) {
            self.should_render = true;
        }

        pub fn game_over(&mut self, score: i64) {
            let total_duration = Instant::now().duration_since(self.start_time);

            self.terminal
                .draw(|frame| {
                    let paragraph = Paragraph::new(Text::styled(
                        r#"
        _      _                     _ 
 /\   /(_) ___| |_ ___  _ __ _   _  / \
 \ \ / / |/ __| __/ _ \| '__| | | |/  /
  \ V /| | (__| || (_) | |  | |_| /\_/ 
   \_/ |_|\___|\__\___/|_|   \__, \/   
                             |___/     
                            "#,
                        Style::default()
                            .add_modifier(Modifier::BOLD)
                            .add_modifier(Modifier::SLOW_BLINK),
                    ))
                    .block(
                        Block::bordered()
                            .border_type(BorderType::Rounded)
                            .title_top(format!(" [ Score: {score} ] "))
                            .title_bottom(format!(
                                " [ {:02}:{:02} ] ",
                                total_duration.as_secs() / 60,
                                total_duration.as_secs() % 60
                            ))
                            .title_alignment(Alignment::Center),
                    );

                    let area = create_centered_layout(frame);
                    let [area] = Layout::vertical([10]).flex(Flex::Center).areas(area);
                    frame.render_widget(paragraph, area)
                })
                .unwrap();

            sleep(Duration::from_secs(5));
        }
    }

    impl Drop for TuiState {
        fn drop(&mut self) {
            ratatui::restore();
        }
    }
}

fn part_2(input: &Input) -> Output2 {
    let mut computer = input.clone();
    computer.write(0, 2);
    let mut input = VecDeque::new();
    let mut output = VecDeque::new();
    let mut score = 0;
    let mut ball_x = -1;
    let mut paddle_x = -1;

    #[cfg(feature = "tui")]
    let mut tui_state = tui::TuiState::init();

    loop {
        match computer.step(&mut SplitIO::new(&mut input, &mut output)) {
            Some(OpCode::Input) => {
                input.push_front(match paddle_x.cmp(&ball_x) {
                    std::cmp::Ordering::Less => 1,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => -1,
                });
                continue;
            }
            Some(OpCode::Terminate) => {
                break;
            }
            _ => {}
        };

        if output.len() == 3 {
            let x = output.pop_front().unwrap();
            let y = output.pop_front().unwrap();
            let tile_id = output.pop_front().unwrap();

            if x == -1 && y == 0 {
                score = tile_id;
            } else {
                let tile = Tile::from_id(tile_id);

                match tile {
                    Tile::HorizontalPaddle => paddle_x = x,
                    Tile::Ball => {
                        #[cfg(feature = "tui")]
                        if ball_x != -1 {
                            tui_state.start_rendering();
                        }
                        ball_x = x;
                    }
                    _ => {}
                }

                #[cfg(feature = "tui")]
                tui_state.update(score, x, y, &tile);
            }
        }
    }

    #[cfg(feature = "tui")]
    tui_state.game_over(score);

    score
}

fn main() {
    env_logger::init();

    log_run("Full run", || {
        let input = log_run("Parsing", || {
            parse(io::stdin().lines().map(|result| result.expect("I/O error")))
        });

        let part_1 = log_run("Part 1", || part_1(&input));
        println!("Part 1: {part_1}");

        let part_2 = log_run("Part 2", || part_2(&input));
        println!("Part 2: {part_2}");
    });
}
