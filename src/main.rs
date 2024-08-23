use std::time::{Duration, Instant};

slint::include_modules!();
#[cfg_attr(target_arch = "wasm32",
           wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
    use slint::Model;

    let main_window = MainWindow::new().unwrap();

    // Fetch the tiles from the model
    let mut tiles: Vec<TileData> = main_window.get_memory_tiles().iter().collect();
    // Duplicate them to ensure that we have pairs
    tiles.extend(tiles.clone());

    // Randomly mix the tiles
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    tiles.shuffle(&mut rng);

    // Assign the shuffled Vec to the model property
    let tiles_model = std::rc::Rc::new(slint::VecModel::from(tiles));
    main_window.set_memory_tiles(tiles_model.clone().into());

    // initialize scores and attempts

    let score = std::rc::Rc::new(std::cell::RefCell::new(0));
    let attempts = std::rc::Rc::new(std::cell::RefCell::new(0));

    let score_clone = score.clone();
    let attempts_clone = attempts.clone();



    let main_window_weak = main_window.as_weak();
    main_window.on_check_if_pair_solved(move || {
        let mut flipped_tiles = tiles_model.iter().enumerate().filter(|(_, tile)| tile.image_visible && !tile.solved);
    
        if let (Some((t1_idx, mut t1)), Some((t2_idx, mut t2))) = (flipped_tiles.next(), flipped_tiles.next())
        {
            // Increment attempts counts
            *attempts_clone.borrow_mut() += 1;
            main_window.set_attempts(*attempts_clone.borrow());

            // Check if pair is solved
            let is_pair_solved = t1 == t2;
            if is_pair_solved {
                // Increment score for correct match
                *score_clone.borrow_mut() += 10;
                main_window.set_score(*score_clone.borrow())

                t1.solved = true;
                tiles_model.set_row_data(t1_idx, t1);
                t2.solved = true;
                tiles_model.set_row_data(t2_idx, t2);

            } else {
                let main_window = main_window_weak.unwrap();
                main_window.set_disable_tiles(true);
                let tiles_model = tiles_model.clone();
                slint::Timer::single_shot(std::time::Duration::from_secs(1), move || {
                    main_window.set_disable_tiles(false);
                    t1.image_visible = false;
                    tiles_model.set_row_data(t1_idx, t1);
                    t2.image_visible = false;
                    tiles_model.set_row_data(t2_idx, t2);
            })
        }
    }
});


    main_window.run().unwrap();
}