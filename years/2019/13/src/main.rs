use std::{
    collections::{HashMap, VecDeque},
    io::{self},
};
#[cfg(feature = "delay")]
use std::{thread::sleep, time::Duration};

use aoc_timing::trace::log_run;
use grid::Grid;
use intcode::{Computer, OpCode, SplitIO};
use itertools::Itertools;

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
    use std::{thread::sleep, time::Duration};

    use super::*;
    use ratatui::{
        DefaultTerminal, Frame,
        layout::{Flex, Layout, Rect},
        style::{Modifier, Style},
        text::Text,
        widgets::{Block, Padding, Paragraph},
    };

    pub struct TuiState {
        terminal: DefaultTerminal,
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

            Self { terminal }
        }

        pub fn update(&mut self, display: &Grid<Tile>, score: i64, ball_moving: bool, tile: &Tile) {
            if ball_moving && tile != &Tile::Empty {
                let display_string = (0..=display.height())
                    .map(|row| {
                        display
                            .row(row)
                            .map(|tile| tile.to_char())
                            .collect::<String>()
                    })
                    .join("\n");

                self.terminal
                    .draw(|frame| {
                        let area = create_centered_layout(frame);
                        let display_text = Paragraph::new(Text::raw(display_string)).block(
                            Block::bordered()
                                .padding(Padding::symmetric(1, 2))
                                .title(format!("Score: {score}"))
                                .title_alignment(ratatui::layout::Alignment::Center),
                        );

                        frame.render_widget(display_text, area);
                    })
                    .unwrap();

                #[cfg(feature = "delay")]
                sleep(Duration::from_millis(50));
            }
        }

        pub fn game_over(&mut self, score: i64) {
            self.terminal
                .draw(|frame| {
                    let area = create_centered_layout(frame);
                    frame.render_widget(
                        Text::styled(
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
                        ),
                        area,
                    )
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
    let mut display: Grid<Tile> =
        std::iter::repeat_n(std::iter::repeat_n(Tile::Empty, 40).collect_vec(), 30).collect();
    let mut ball_x = -1;
    let mut paddle_x = -1;
    #[cfg(feature = "tui")]
    let mut ball_moving = false;

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
                display.update(x as usize, y as usize, tile);

                match tile {
                    Tile::HorizontalPaddle => paddle_x = x,
                    Tile::Ball => {
                        #[cfg(feature = "tui")]
                        if ball_x != -1 {
                            ball_moving = true;
                        }
                        ball_x = x;
                    }
                    _ => {}
                }

                #[cfg(feature = "tui")]
                tui_state.update(&display, score, ball_moving, &tile);
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
