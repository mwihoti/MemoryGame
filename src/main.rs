use std::time::Instant;
use std::convert::TryInto;
use rand::prelude::SliceRandom; // Import the SliceRandom trait for shuffle method

slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
    use slint::Model;

    let main_window = MainWindow::new().unwrap();

    // Initialize timer and other components
    let start_time = Instant::now();
    let timer_active = std::rc::Rc::new(std::cell::RefCell::new(true));
    let timer_active_clone = timer_active.clone();
    let main_window_weak = main_window.as_weak();

    // Create a Timer instance
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

    // Initialize tiles for the game
    let mut tiles: Vec<TileData> = main_window.get_memory_tiles().iter().collect();
    tiles.extend(tiles.clone());
    tiles.shuffle(&mut rand::thread_rng()); // Shuffle tiles randomly

    // Assign the shuffled Vec to the model property
    let tiles_model = std::rc::Rc::new(slint::VecModel::from(tiles));
    main_window.set_memory_tiles(tiles_model.clone().into());

    // Initialize scores and attempts
    let score = std::rc::Rc::new(std::cell::RefCell::new(0));
    let attempts = std::rc::Rc::new(std::cell::RefCell::new(0));

    let score_clone = score.clone();
    let attempts_clone = attempts.clone();

    let tiles_model_clone = tiles_model.clone();
    let main_window_weak = main_window.as_weak();

    main_window.on_check_if_pair_solved(move || {
        let flipped_tiles: Vec<(usize, TileData)> = tiles_model_clone.iter().enumerate()
            .filter(|(_, tile)| tile.image_visible && !tile.solved)
            .collect();
        
        if flipped_tiles.len() == 2 {
            let (t1_idx, t1) = flipped_tiles[0].clone();
            let (t2_idx, t2) = flipped_tiles[1].clone();
        
            // Increment attempts
            *attempts_clone.borrow_mut() += 1;
            if let Some(main_window) = main_window_weak.upgrade() {
                main_window.set_attempts(*attempts_clone.borrow());
            }
        
            // Check if pair is solved
            if t1.image == t2.image {
                // Increment score for correct match
                *score_clone.borrow_mut() += 10;
                if let Some(main_window) = main_window_weak.upgrade() {
                    main_window.set_score(*score_clone.borrow());
                }
        
                let mut t1 = tiles_model_clone.row_data(t1_idx).unwrap();  // Dereference t1_idx
                let mut t2 = tiles_model_clone.row_data(t2_idx).unwrap();  // Dereference t2_idx
                t1.solved = true;
                t2.solved = true;
                tiles_model_clone.set_row_data(t1_idx, t1);  // Dereference t1_idx
                tiles_model_clone.set_row_data(t2_idx, t2);  // Dereference t2_idx
            } else {
                if let Some(main_window) = main_window_weak.upgrade() {
                    main_window.set_disable_tiles(true);
                }

                let tiles_model_clone_inner = tiles_model_clone.clone();
                let main_window_weak_inner = main_window_weak.clone();
                slint::Timer::single_shot(std::time::Duration::from_secs(1), move || {
                    if let Some(main_window) = main_window_weak_inner.upgrade() {
                        let mut t1 = tiles_model_clone_inner.row_data(t1_idx).unwrap();  // Dereference t1_idx
                        let mut t2 = tiles_model_clone_inner.row_data(t2_idx).unwrap();  // Dereference t2_idx
                        t1.image_visible = false;
                        t2.image_visible = false;
                        tiles_model_clone_inner.set_row_data(t1_idx, t1);  // Dereference t1_idx
                        tiles_model_clone_inner.set_row_data(t2_idx, t2);  // Dereference t2_idx
                        main_window.set_disable_tiles(false);
                    }
                });
            }
        }
    });

    main_window.run().unwrap();
}
