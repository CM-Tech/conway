use std::{thread, io};
use std::io::BufRead;
use std::{cmp, fmt};
use std::time::Duration;

const MAP_WIDTH: usize = 40;
const MAP_HEIGHT: usize = 30;

struct Conway {
    map: [[bool; MAP_WIDTH]; MAP_HEIGHT],
}

impl Conway {
    fn new(pattern: Vec<&'static str>) -> Conway {
        let mut map = [[false; MAP_WIDTH]; MAP_HEIGHT];
        let h0 = (MAP_HEIGHT - pattern.len()) / 2;
        for i in 0..pattern.len() {
            let row = pattern[i];
            let w = row.len();
            let w0 = (MAP_WIDTH - w) / 2;
            for (j, c) in row.chars().enumerate() {
                map[i + h0][j + w0] = c == '#';
            }
        }
        Conway { map: map }
    }

    fn next(&mut self) {
        let mut newmap = [[false; MAP_WIDTH]; MAP_HEIGHT];
        for i in 0..MAP_HEIGHT {
            for j in 0..MAP_WIDTH {
                let mut nlive = 0;
                for i2 in i.saturating_sub(1)..cmp::min(i + 2, MAP_HEIGHT) {
                    for j2 in j.saturating_sub(1)..cmp::min(j + 2, MAP_WIDTH) {
                        if self.map[i2][j2] && (i2 != i || j2 != j) {
                            nlive += 1;
                        }
                    }
                }
                newmap[i][j] = match (self.map[i][j], nlive) {
                    (true, 2) | (_, 3) => true,
                    _ => false,
                };
            }
        }
        self.map = newmap;
    }
}

impl fmt::Display for Conway {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "┌{}┐\n", "──".repeat(MAP_WIDTH));
        for row in self.map.iter() {
            write!(f, "│");
            for cell in row.iter() {
                write!(f, "{}", if *cell { "██" } else { "  " });
            }
            write!(f, "│\n");
        }
        write!(f, "└{}┘\n", "──".repeat(MAP_WIDTH));
        Ok(())
    }
}

fn main() {
    thread::spawn(|| {
                      let stdin = io::stdin();
                      for _ in stdin.lock().lines() {
                          std::process::exit(0);
                      }
                  });

    let mut game = Conway::new(vec!["                        #           ",
                                    "                      # #           ",
                                    "            ##      ##            ##",
                                    "           #   #    ##            ##",
                                    "##        #     #   ##              ",
                                    "##        #   # ##    # #           ",
                                    "          #     #       #           ",
                                    "           #   #                    ",
                                    "            ##                      "]);

    print!("\x1b[2J");
    for i in 1.. {
        print!("\x1b[H{}", game);
        println!("n = {:<5} Press ENTER to exit", i);
        thread::sleep(Duration::from_millis(20));
        game.next();
    }
}
