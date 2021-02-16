use rand::Rng;
use std::thread;
use std::sync::mpsc;


// CONSTANTS:
const SIMULATIONS: usize = 10_000_000;
const N_DECKS: u32 = 4;
const N_THREADS: usize = 10;


fn shuffle_deck(deck: &mut Vec<u32>, rng: &mut rand::prelude::ThreadRng) {
    let mut idx1: usize;
    let mut idx2: usize;
    let mut parking_space: u32;
    for _ in 0..1000 {
        idx1 = rng.gen_range(0..deck.len() as u32) as usize;
        idx2 = rng.gen_range(0..deck.len() as u32) as usize;

        parking_space = deck[idx1];
        deck[idx1] = deck[idx2];
        deck[idx2] = parking_space;
    }
}


fn reset_deck(deck: &mut Vec<u32>) {
    for _ in 0..N_DECKS {
        // loop over card decks
        for num in [2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11].iter() {
            // loop over card numbs
            for _i in 0..4 {
                // Add all four colors
                deck.push(*num);
            }
        }
    }
}


fn sum(deck: &Vec<u32>) -> u32 {
    let mut total: u32 = 0;

    for card in deck.iter() {
        total += card;
    }

    total
}


fn simulate_games(n_games: usize) -> [usize; 3] {
    let mut deck: Vec<u32> = Vec::new();

    reset_deck(&mut deck);

    // Object needed for shuffeling
    let mut rng = rand::thread_rng();
    shuffle_deck(&mut deck, &mut rng);

    // keep score
    let mut p_wins: usize = 0;
    let mut d_wins: usize = 0;
    let mut ties: usize = 0;

    for match_n in 0..n_games {
        if deck.len() < (N_DECKS * 4 * 13) as usize / 4 {
            println!("Working on match: {} / {} = {:.2}%", match_n, n_games, (match_n as f64 / n_games as f64) * 100.0);

            reset_deck(&mut deck);
            shuffle_deck(&mut deck, &mut rng);
        }

        let mut p_deck: Vec<u32> = Vec::new();
        let mut d_deck: Vec<u32> = Vec::new();
        let mut new_card: u32;
        let mut p_deck_sum: u32;
        let mut d_deck_sum: u32;

        // deal player deck
        loop {
            new_card = deck.pop().unwrap();
            p_deck.push(new_card);

            p_deck_sum = sum(&p_deck);

            if p_deck_sum > 16 {
                break
            }
        }

        // convert aces to 1's if necessary
        if p_deck_sum > 21 {
            for idx in 0..p_deck.len() as usize {
                if p_deck[idx] == 11 {
                    p_deck[idx] = 1;
                    break;
                }
            }
        }

        // deal dealer deck
        loop {
            new_card = deck.pop().unwrap();
            d_deck.push(new_card);

            d_deck_sum = sum(&d_deck);

            if d_deck_sum >= 17 {
                break
            }
        }

        // convert aces to 1's if necessary
        if d_deck_sum > 21 {
            for idx in 0..d_deck.len() as usize {
                if d_deck[idx] == 11 {
                    d_deck[idx] = 1;
                    break;
                }
            }
        }

        p_deck_sum = sum(&p_deck);
        d_deck_sum = sum(&d_deck);

        // check for winner
        if d_deck_sum > 21 {
            p_wins += 1;
        } else if p_deck_sum == d_deck_sum {
            ties += 1;
        } else if p_deck_sum > 21 {
            d_wins += 1;
        } else if p_deck_sum > d_deck_sum {
            p_wins += 1;
        } else if d_deck_sum > p_deck_sum {
            d_wins += 1;
        }
    }

    [p_wins, ties, d_wins]
}


fn main() {
    // thread communication
    let (tx, rx) = mpsc::channel::<[usize; 3]>();

    for _n in 0..N_THREADS {
        let tx1 = tx.clone();
        thread::spawn(move || {
            let result: [usize; 3] = simulate_games(SIMULATIONS / N_THREADS);
            tx1.send(result).unwrap();
        });
    }

    let mut p_wins: usize = 0;
    let mut ties: usize = 0;
    let mut d_wins: usize = 0;

    let mut recieved_messages = 0;
    for message in rx {
        println!("{:?}", message);

        p_wins += message[0];
        ties += message[1];
        d_wins += message[2];

        recieved_messages += 1;
        if recieved_messages == N_THREADS {
            break;
        }
    }

    let win_percentage = (p_wins as f64 / SIMULATIONS as f64) * 100.0;
    let tie_percentage = (ties as f64 / SIMULATIONS as f64) * 100.0;
    let lost_percentage = (d_wins as f64 / SIMULATIONS as f64) * 100.0;

    println!("\n[== Simulation done ==]\n");
    println!("     |number    |percentage");
    println!("-----+----------+----------");
    println!("Wins |{: <10}|{:.2}%\nTies |{: <10}|{:.2}%\nLost |{: <10}|{:.2}%", p_wins, win_percentage, ties, tie_percentage, d_wins, lost_percentage);

    println!("\nGain: {}$", p_wins as isize - d_wins as isize);
}
