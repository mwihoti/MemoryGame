use std::time::Instant;
use std::convert::TryInto;
use rand::prelude::SliceRandom;
use slint::Model;
use std::rc::Rc;

slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
    let main_window = MainWindow::new().unwrap();

    // Initialize player name
    

    // Initialize timer and other components
    let start_time = Instant::now();
    let timer_active = Rc::new(std::cell::RefCell::new(true));
    let timer_active_clone = timer_active.clone();
    let main_window_weak = main_window.as_weak();

    let game_timer = slint::Timer::default();
    game_timer.start(
        slint::TimerMode::Repeated,
        std::time::Duration::from_secs(1),
        move || {
            if let Some(main_window) = main_window_weak.upgrade() {
                if *timer_active_clone.borrow() {
                    let elapsed = start_time.elapsed();
                    let seconds = elapsed.as_secs();
                    main_window.set_time_elapsed(seconds.try_into().unwrap());
                }
            }
        },
    );

    main_window.on_pause_resume(move || {
        let mut active = timer_active.borrow_mut();
        *active = !*active;
    });

    let mut tiles: Vec<TileData> = main_window.get_memory_tiles().iter().collect();
    tiles.extend(tiles.clone());
    tiles.shuffle(&mut rand::thread_rng());

    let tiles_model = Rc::new(slint::VecModel::from(tiles));
    main_window.set_memory_tiles(tiles_model.clone().into());

    let score = Rc::new(std::cell::RefCell::new(0));
    let attempts = Rc::new(std::cell::RefCell::new(0));
    let tile_flips_in_progress = Rc::new(std::cell::RefCell::new(false)); // Track flipping state

    let score_clone = score.clone();
    let attempts_clone = attempts.clone();
    let  tiles_model_clone = tiles_model.clone();
    let main_window_weak2 = main_window.as_weak();
    let tile_flips_in_progress_clone = tile_flips_in_progress.clone();

    let check_if_pair_solved: Rc<Box<dyn Fn()>> = Rc::new(Box::new({
        let score_clone = score_clone.clone();
        let attempts_clone = attempts_clone.clone();
        let tiles_model_clone = tiles_model_clone.clone();
        let main_window_weak2 = main_window_weak2.clone();
        let tile_flips_in_progress_clone = tile_flips_in_progress_clone.clone();
        
        
        move || {
            if *tile_flips_in_progress_clone.borrow() {
                return; // Prevent checking if a flip process is already in progress
            }

            let flipped_tiles: Vec<(usize, TileData)> = tiles_model_clone
                .iter()
                .enumerate()
                .filter(|(_, tile)| tile.image_visible && !tile.solved)
                .collect();

            if flipped_tiles.len() == 2 {
                *tile_flips_in_progress_clone.borrow_mut() = true; // Set flip process as in progress

                let (t1_idx, t1) = flipped_tiles[0].clone();
                let (t2_idx, t2) = flipped_tiles[1].clone();

                *attempts_clone.borrow_mut() += 1;
                if let Some(main_window) = main_window_weak2.upgrade() {
                    main_window.set_attempts(*attempts_clone.borrow());
                }

                if t1.image == t2.image {
                    *score_clone.borrow_mut() += 10;
                    if *score_clone.borrow() == (tiles_model_clone.row_count() / 2) * 10 {
                      //  show_congratulations_message(
                         //   &main_window_weak2,
                       //     (*score_clone.borrow()).try_into().unwrap(),
                        
                    //    );
                    }
                    if let Some(main_window) = main_window_weak2.upgrade() {
                        main_window.set_score((*score_clone.borrow()).try_into().unwrap());
                    }

                    let mut t1 = tiles_model_clone.row_data(t1_idx).unwrap();
                    let mut t2 = tiles_model_clone.row_data(t2_idx).unwrap();
                    t1.solved = true;
                    t2.solved = true;
                    tiles_model_clone.set_row_data(t1_idx, t1);
                    tiles_model_clone.set_row_data(t2_idx, t2);

                    *tile_flips_in_progress_clone.borrow_mut() = false; // Reset flip process status
                } else {
                    if let Some(main_window) = main_window_weak2.upgrade() {
                        main_window.set_disable_tiles(true);
                    }
                      
                    let tiles_model_clone_inner = tiles_model_clone.clone();
                    let main_window_weak_inner = main_window_weak2.clone();
                    let tile_flips_in_progress_clone_inner = tile_flips_in_progress_clone.clone();

                    slint::Timer::single_shot(std::time::Duration::from_secs(1), move || {
                        if let Some(main_window) = main_window_weak_inner.upgrade() {
                            let mut t1 = tiles_model_clone_inner.row_data(t1_idx).unwrap();
                            let mut t2 = tiles_model_clone_inner.row_data(t2_idx).unwrap();
                            t1.image_visible = false;
                            t2.image_visible = false;
                            tiles_model_clone_inner.set_row_data(t1_idx, t1);
                            tiles_model_clone_inner.set_row_data(t2_idx, t2);
                            main_window.set_disable_tiles(false);
                            *tile_flips_in_progress_clone_inner.borrow_mut() = false; // Reset flip process status
                        }
                    });
                }
            }
        }
    }));

    main_window.on_check_if_pair_solved({
        let check_if_pair_solved_clone = check_if_pair_solved.clone();
        move || {
            (check_if_pair_solved_clone)();
        }
    });

    let main_window_weak_3 = main_window.as_weak();
    main_window.on_reset_game({
        let check_if_pair_solved_clone = check_if_pair_solved.clone();
        move || {
            *score.borrow_mut() = 0;
            *attempts.borrow_mut() = 0;
            // Reset flip process status
            check_if_pair_solved_clone.clone();
            restart_game(&main_window_weak_3);
        }
    });

    main_window.run().unwrap();
}

fn show_congratulations_message(
    main_window: &slint::Weak<MainWindow>,
    score: i32,
    check_if_pair_solved: Rc<Box<dyn Fn()>>,
) {
    if let Some(main_window) = main_window.upgrade() {
        main_window.set_congratulations_message(
            format!("Congratulations! You won with a score of {}.", score).into(),
        );
        reshuffle_tiles(&main_window);
    }
}
fn restart_game(main_window: &slint::Weak<MainWindow>) {
    if let Some(main_window) = main_window.upgrade() {
        reshuffle_tiles(&main_window);
       
    }
}


fn reshuffle_tiles(main_window: &MainWindow) {
    // Reset score, attempts, and other game state
   // main_window.set_score(0);
   // main_window.set_attempts(0);
   // main_window.set_time_elapsed(0);
   // main_window.set_disable_tiles(false);
    main_window.set_congratulations_message("".into());

    // Collect the current tiles and prepare for reshuffling
    let mut tiles: Vec<TileData> = main_window.get_memory_tiles().iter().collect();
    for tile in &mut tiles {
        tile.image_visible = false;
        tile.solved = false;
    }

    // Shuffle the tiles
    tiles.shuffle(&mut rand::thread_rng());

    // Set the reshuffled tiles back to the main window
    let tiles_model = Rc::new(slint::VecModel::from(tiles));
    let tiles_model_clone = tiles_model.clone();
    main_window.set_memory_tiles(tiles_model.into());

     // Ensure that no more than two tiles are flipped at the same time
    let flipped_tiles: Vec<(usize, TileData)> = tiles_model_clone
        .iter()
        .enumerate()
        .filter(|(_, tile)| tile.image_visible && !tile.solved)
        .collect();

    if flipped_tiles.len() > 2 {
        // If more than two tiles are flipped, flip them back
        for (idx, mut tile) in flipped_tiles {
            tile.image_visible = false;
            tiles_model_clone.set_row_data(idx, tile);
        }
    }

    
}

