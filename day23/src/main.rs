// Quick and dirty, should clean up when I have the proper time

use cached::proc_macro::cached;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct State {
    hallway: [char; 7],
    rooms: [[char; 4]; 4],
}

impl State {
    // Cost of move + new state
    fn get_moves(&self) -> Vec<(State, usize)> {
        let mut moves = Vec::default();
        let val = |c: char| match c {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => unreachable!(),
        };
        let where_to = |c: char| match c {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            _ => unreachable!(),
        };

        // Move from room to hallway
        for (room_idx, room) in self.rooms.iter().enumerate() {
            for (char_idx, &char) in room.iter().enumerate() {
                // No character, or no path, or room is correct
                if char == 'E'
                    || !room[0..char_idx].iter().all(|&c| c == 'E')
                    || (room.iter().all(|&c| c == 'E' || where_to(c) == room_idx))
                {
                    continue;
                }

                let dirs: [Vec<usize>; 2] = [
                    ((0..room_idx + 2).rev()).collect(),
                    (room_idx + 2..self.hallway.len()).collect(),
                ];
                let mut steps;
                for dir in dirs {
                    steps = 0;
                    for hallway_idx in dir {
                        steps += 2;

                        if self.hallway[hallway_idx] != 'E' {
                            break;
                        };
                        let mut newhall = self.hallway;
                        let mut newrooms = self.rooms;
                        newhall[hallway_idx] = char;
                        newrooms[room_idx][char_idx] = 'E';
                        moves.push((
                            State {
                                hallway: newhall,
                                rooms: newrooms,
                            },
                            (steps
                                - if hallway_idx == 0 || hallway_idx == newhall.len() - 1 {
                                    1
                                } else {
                                    0
                                }
                                + char_idx)
                                * val(char),
                        ));
                    }
                }
            }
        }

        // Move from hallway into room (only the correct one)
        'hall: for (hallway_idx, &char) in self.hallway.iter().enumerate() {
            // Is not movable?
            if char == 'E' {
                continue;
            };

            let room_idx = where_to(char);

            // Room not legal?
            for c in self.rooms[room_idx] {
                if c != char && c != 'E' {
                    continue 'hall;
                }
            }

            let range: Vec<usize> = if room_idx + 2 <= hallway_idx {
                (room_idx + 2..hallway_idx).rev().collect()
            } else {
                (hallway_idx + 1..room_idx + 2).collect()
            };

            let mut steps = 2;

            for hall_check_idx in range {
                if self.hallway[hall_check_idx] != 'E' {
                    continue 'hall;
                }
                steps += 2;
            }

            if hallway_idx == 0 || hallway_idx == self.hallway.len() - 1 {
                steps -= 1;
            }

            let mut newhall = self.hallway;
            let mut newrooms = self.rooms;
            let mut newroom = newrooms[room_idx];

            let insertion_idx = (0..newroom.len())
                .rev()
                .find(|&idx| newroom[idx] == 'E')
                .unwrap();

            steps += insertion_idx;

            newroom[insertion_idx] = char;
            newrooms[room_idx] = newroom;
            newhall[hallway_idx] = 'E';

            moves.push((
                State {
                    hallway: newhall,
                    rooms: newrooms,
                },
                steps * val(char),
            ));
        }

        moves
    }
}

#[cached]
fn get_energy_cost(state: State, total: usize) -> usize {
    let where_to = |c: char| match c {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 100,
        _ => unreachable!(),
    };

    // Where can we move?
    let moves = state.get_moves();

    // We reached the goal if we can move no longer and everything is in it's right place
    if moves.is_empty() {
        if state
            .rooms
            .iter()
            .enumerate()
            .all(|(room_idx, room)| room.iter().all(|&c| where_to(c) == room_idx))
        {
            return total;
        } else {
            return usize::MAX;
        }
    }

    moves
        .iter()
        .map(|&(state, cost)| get_energy_cost(state, total + cost))
        .min()
        .unwrap()
}

fn main() {
    // --- Uncomment for Part 1 input ---
    // let input = State {
    //     hallway: ['E', 'E', 'E', 'E', 'E', 'E', 'E'],
    //     rooms: [['C', 'C'], ['A', 'A'], ['B', 'D'], ['D', 'B']],
    // };

    let input = State {
        hallway: ['E', 'E', 'E', 'E', 'E', 'E', 'E'],
        rooms: [
            ['C', 'D', 'D', 'C'],
            ['A', 'C', 'B', 'A'],
            ['B', 'B', 'A', 'D'],
            ['D', 'A', 'C', 'B'],
        ],
    };

    println!("Answer: {}", get_energy_cost(input, 0));
}
