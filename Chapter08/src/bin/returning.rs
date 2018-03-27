extern crate futures;

use futures::executor::block_on;
use futures::future::{join_all, Future, FutureResult, ok};
use futures::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum PlayerStatus {
    Loading,
    Default,
    Jumping,
}

#[derive(Clone, Copy, Debug)]
struct Player {
    name: &'static str,
    status: PlayerStatus,
    score: u32,
    ticks: usize,
}

impl Player {
    fn new(name: &'static str) -> Self {
        let mut ticks = 1;
        // Give Bob more ticks explicitly
        if name == "Bob" {
            ticks = 5;
        }

        Player {
            name: name,
            status: PlayerStatus::Loading,
            score: 0,
            ticks: ticks,
        }
    }

    fn set_status(&mut self, status: PlayerStatus) -> FutureResult<&mut Self, Never> {
        self.status = status;
        ok(self)
    }

    fn can_add_points(&mut self) -> bool {
        if self.status == PlayerStatus::Default {
            return true;
        }

        println!("We couldn't add any points for {}!", self.name);
        return false;
    }

    fn add_points(&mut self, points: u32) -> Async<&mut Self> {
        if !self.can_add_points() {
            Async::Ready(self)
        } else {
            let new_score = self.score + points;
            // Here we would send the new score to a remote server
            // but for now we will manaully increment the player's score.

            self.score = new_score;

            Async::Ready(self)
        }
    }
}

impl Future for Player {
    type Item = Player;
    type Error = ();

    fn poll(&mut self, cx: &mut task::Context) -> Poll<Self::Item, Self::Error> {
        // Presuming we fetch our player's score from a
        // server upon initial load.
        // After we perform the fetch send the Result<Async> value.

        println!("Player {} has been poll'ed!", self.name);

        if self.ticks == 0 {
            self.status = PlayerStatus::Default;
            Ok(Async::Ready(*self))
        } else {
            self.ticks -= 1;
            cx.waker().wake();
            Ok(Async::Pending)
        }
    }
}

fn async_add_points(player: &mut Player,
                    points: u32)
                    -> Box<Future<Item = Player, Error = Never> + Send> {
    // Presuming that player.add_points() will send the points to a
    // database/server over a network and returns an updated
    // player score from the server/database.
    let _ = player.add_points(points);

    // Additionally, we may want to add logging mechanisms,
    // friend notifications, etc. here.

    return Box::new(ok(*player));
}

fn display_scoreboard(players: Vec<&Player>) {
    for player in players {
        println!("{}'s Score: {}", player.name, player.score);
    }
}

fn main() {
    let mut player1 = Player::new("Bob");
    let mut player2 = Player::new("Alice");

    let tasks = join_all(vec![player1, player2]);

    let f = join_all(vec![
        async_add_points(&mut player1, 5),
        async_add_points(&mut player2, 2),
    ])
        .then(|x| {
            println!("First batch of adding points is done.");
            x
        });

    block_on(f).unwrap();

    let players = block_on(tasks).unwrap();
    player1 = players[0];
    player2 = players[1];

    println!("Scores should be zero since no players were loaded");
    display_scoreboard(vec![&player1, &player2]);

    // In our minigame, a player cannot score if they are currently
    // in the air or "jumping."
    // Let's make one of our players' status set to the jumping status.

    let f = player2.set_status(PlayerStatus::Jumping).and_then(move |mut new_player2| {
        async_add_points(&mut player1, 10)
            .and_then(move |_| {
                println!("Finished trying to give Player 1 points.");
                async_add_points(&mut new_player2, 2)
            })
            .then(move |new_player2| {
                println!("Finished trying to give Player 2 points.");
                println!("Player 1 (Bob) should have a score of 10 and Player 2 (Alice) should \
                          have a score of 0");

                // unwrap is used here to since
                display_scoreboard(vec![&player1, &new_player2.unwrap()]);
                new_player2
            })
    });

    block_on(f).unwrap();

    println!("All done!");
}
