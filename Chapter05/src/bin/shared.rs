use std::rc::Rc;

// The ball will survive until all kids are done playing with it
struct Kid {
    ball: Rc<Ball>,
}
struct Ball;

fn main() {
    {
        // rc is created and count is at 1
        let foo = Rc::new("foo");
        // foo goes out of scope; count decreases
        // count is zero; the object gets destroyed
    }

    {
        // rc is created and count is at 1
        let bar = Rc::new("bar");
        // rc is cloned; count increases to 2
        let second_bar = Rc::clone(&bar);
        // bar goes out of scode; count decreases to 1
        // bar goes out of scode; count decreases to 0
    }

    {
        // rc is created and count is at 1
        let baz = Rc::new("baz");
        {
            // rc is cloned; count increases to 2
            let second_baz = Rc::clone(&baz);
            // second_baz goes out of scode; count decreases to 1
        }
        // baz goes out of scode; count decreases to 0
    }
    let kid_one = spawn_kid_with_new_ball();
    let kid_two = Kid {
        ball: Rc::clone(&kid_one.ball),
    };
    let kid_three = Kid {
        ball: Rc::clone(&kid_one.ball),
    };
    // ball lives until here
}


fn spawn_kid_with_new_ball() -> Kid {
    let ball = Rc::new(Ball);
    Kid {
        ball: Rc::clone(&ball),
    }
    // Although the ball goes out of scope here, the object behind it
    // will survive as part of the kid
}
