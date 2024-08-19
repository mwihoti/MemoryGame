
slint::include_modules!();

pub fn main() {
    use slint::Model;
    use rand::seq::SliceRandom;
    use std::rc::Rc;

    // Define the TileData struct matching the one in Slint UI
    #[derive(Clone, PartialEq)]
    struct TileData {
        image: slint::Image,
        image_visible: bool,
        solved: bool,
    }

    // Initialize the main window
    let main_window = MainWindow::new().unwrap();

    // Fetch the tiles from the model
    let mut tiles: Vec<TileData> = main_window.get_memory_tiles().iter().map(|tile| TileData {
        image: tile.image.clone(),
        image_visible: tile.image_visible,
        solved: tile.solved,
    }).collect();

    // Duplicate the tiles to create pairs and shuffle them
    tiles.extend(tiles.clone());
    let mut rng = rand::thread_rng();
    tiles.shuffle(&mut rng);

    // Assign the shuffled Vec to the model property
    let tiles_model = Rc::new(slint::VecModel::from(tiles));
    main_window.set_memory_tiles(tiles_model.clone().into());

    // Handle the game logic for checking pairs
    let main_window_weak = main_window.as_weak();
    main_window.on_check_if_pair_solved(move || {
        let mut flipped_tiles = tiles_model.iter().enumerate()
            .filter(|(_, tile)| tile.image_visible && !tile.solved);

        if let (Some((t1_idx, mut t1)), Some((t2_idx, mut t2))) = 
            (flipped_tiles.next(), flipped_tiles.next()) {
            let is_pair_solved = t1.image == t2.image;
            if is_pair_solved {
                t1.solved = true;
                t2.solved = true;
                tiles_model.set_row_data(t1_idx, t1);
                tiles_model.set_row_data(t2_idx, t2);
            } else {
                let main_window = main_window_weak.unwrap();
                main_window.set_disable_tiles(true);
                slint::Timer::single_shot(std::time::Duration::from_secs(1), move || {
                    main_window.set_disable_tiles(false);
                    t1.image_visible = false;
                    t2.image_visible = false;
                    tiles_model.set_row_data(t1_idx, t1);
                    tiles_model.set_row_data(t2_idx, t2);
                });
            }
        }
    });

    main_window.run().unwrap();
}
