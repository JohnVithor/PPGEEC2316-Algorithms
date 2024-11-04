use std::collections::{BTreeMap, HashMap, LinkedList, VecDeque};

#[derive(Debug, PartialEq)]
enum FirstOrLast {
    First,
    Last,
    Other,
}

#[derive(Debug)]
struct Photo {
    id: i32,
}

trait Gallery {
    fn add_photo(&mut self, photo: Photo);
    fn remove_photo(&mut self, id: i32, pos: usize, relative_pos: FirstOrLast) -> Option<Photo>;
    fn get_photo(&self, id: i32, pos: usize, relative_pos: FirstOrLast) -> Option<&Photo>;
}

struct VecGallery {
    photos: Vec<Photo>,
}

struct VecDequeGallery {
    photos: VecDeque<Photo>,
}

struct LinkedListGallery {
    photos: LinkedList<Photo>,
}

struct HashGallery {
    photos: HashMap<i32, Photo>,
}

struct BTreeGallery {
    photos: BTreeMap<i32, Photo>,
}

impl Gallery for VecGallery {
    fn add_photo(&mut self, photo: Photo) {
        self.photos.push(photo);
    }

    fn remove_photo(&mut self, _id: i32, pos: usize, relative_pos: FirstOrLast) -> Option<Photo> {
        if relative_pos == FirstOrLast::Last {
            return self.photos.pop();
        }
        Some(self.photos.remove(pos))
    }

    fn get_photo(&self, _id: i32, pos: usize, relative_pos: FirstOrLast) -> Option<&Photo> {
        if relative_pos == FirstOrLast::Last {
            return self.photos.last();
        }
        self.photos.get(pos)
    }
}

impl Gallery for VecDequeGallery {
    fn add_photo(&mut self, photo: Photo) {
        self.photos.push_back(photo);
    }

    fn remove_photo(&mut self, _id: i32, pos: usize, relative_pos: FirstOrLast) -> Option<Photo> {
        match relative_pos {
            FirstOrLast::First => self.photos.pop_front(),
            FirstOrLast::Last => self.photos.pop_back(),
            FirstOrLast::Other => self.photos.remove(pos),
        }
    }

    fn get_photo(&self, _id: i32, pos: usize, relative_pos: FirstOrLast) -> Option<&Photo> {
        match relative_pos {
            FirstOrLast::First => self.photos.front(),
            FirstOrLast::Last => self.photos.back(),
            FirstOrLast::Other => self.photos.get(pos),
        }
    }
}

impl Gallery for LinkedListGallery {
    fn add_photo(&mut self, photo: Photo) {
        self.photos.push_back(photo);
    }

    fn remove_photo(&mut self, _id: i32, pos: usize, relative_pos: FirstOrLast) -> Option<Photo> {
        match relative_pos {
            FirstOrLast::First => self.photos.pop_front(),
            FirstOrLast::Last => self.photos.pop_back(),
            FirstOrLast::Other => {
                let mut lists = self.photos.split_off(pos);
                let photo = lists.pop_front()?;
                self.photos.append(&mut lists);
                Some(photo)
            }
        }
    }

    fn get_photo(&self, _id: i32, pos: usize, relative_pos: FirstOrLast) -> Option<&Photo> {
        match relative_pos {
            FirstOrLast::First => self.photos.front(),
            FirstOrLast::Last => self.photos.back(),
            FirstOrLast::Other => self.photos.iter().nth(pos),
        }
    }
}

impl Gallery for HashGallery {
    fn add_photo(&mut self, photo: Photo) {
        self.photos.insert(photo.id, photo);
    }

    fn remove_photo(&mut self, id: i32, _pos: usize, _relative_pos: FirstOrLast) -> Option<Photo> {
        self.photos.remove(&id)
    }

    fn get_photo(&self, id: i32, _pos: usize, _relative_pos: FirstOrLast) -> Option<&Photo> {
        self.photos.get(&id)
    }
}

impl Gallery for BTreeGallery {
    fn add_photo(&mut self, photo: Photo) {
        self.photos.insert(photo.id, photo);
    }

    fn remove_photo(&mut self, id: i32, _pos: usize, _relative_pos: FirstOrLast) -> Option<Photo> {
        self.photos.remove(&id)
    }

    fn get_photo(&self, id: i32, _pos: usize, _relative_pos: FirstOrLast) -> Option<&Photo> {
        self.photos.get(&id)
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let mut current_names = Vec::new();

    let mut insert_counter = 0;
    let mut remove_first_counter = 0;
    let mut remove_last_counter = 0;
    let mut remove_rand_counter = 0;
    let mut search_first_counter = 0;
    let mut search_last_counter = 0;
    let mut search_rand_counter = 0;

    let mut insert = 0.0;
    let mut remove_first = 0.0;
    let mut remove_last = 0.0;
    let mut remove_rand = 0.0;
    let mut search_first = 0.0;
    let mut search_last = 0.0;
    let mut search_rand = 0.0;

    let n = args[2]
        .parse::<usize>()
        .expect("Invalid number of operations to execute");

    let mut gallery: Box<dyn Gallery> = match args[1].as_str() {
        "vec" => Box::new(VecGallery {
            photos: Vec::with_capacity(n),
        }),
        "vecdeque" => Box::new(VecDequeGallery {
            photos: VecDeque::with_capacity(n),
        }),
        "hash" => Box::new(HashGallery {
            photos: HashMap::with_capacity(n),
        }),
        "btree" => Box::new(BTreeGallery {
            photos: BTreeMap::new(),
        }),
        "linked" => Box::new(LinkedListGallery {
            photos: LinkedList::new(),
        }),
        _ => panic!("Invalid gallery type"),
    };

    let seed = args[3]
        .parse::<u64>()
        .expect("Invalid seed for random number generator");
    fastrand::seed(seed);
    let total = std::time::Instant::now();
    for _n in 0..n {
        let op = if current_names.is_empty() {
            0
        } else {
            fastrand::u8(..=2)
        };

        match op {
            0 => {
                let start = std::time::Instant::now();
                let photo = Photo {
                    id: fastrand::i32(..),
                };
                current_names.push(photo.id);
                gallery.add_photo(photo);
                let end = start.elapsed().as_nanos();
                insert += end as f64;
                insert_counter += 1;
            }
            1 => {
                let remove_type = fastrand::u8(..=2);
                let id = match remove_type {
                    0 => {
                        let start = std::time::Instant::now();
                        let _photo = gallery.remove_photo(current_names[0], 0, FirstOrLast::First);
                        let end = start.elapsed().as_nanos();
                        remove_first += end as f64;
                        remove_first_counter += 1;
                        0
                    }
                    1 => {
                        let start = std::time::Instant::now();
                        let _photo = gallery.remove_photo(
                            current_names[current_names.len() - 1],
                            current_names.len() - 1,
                            FirstOrLast::Last,
                        );
                        let end = start.elapsed().as_nanos();
                        remove_last += end as f64;
                        remove_last_counter += 1;
                        current_names.len() - 1
                    }
                    _ => {
                        let start = std::time::Instant::now();
                        let id = fastrand::usize(0..current_names.len());
                        let _photo =
                            gallery.remove_photo(current_names[id], id, FirstOrLast::Other);
                        let end = start.elapsed().as_nanos();
                        remove_rand += end as f64;
                        remove_rand_counter += 1;
                        id
                    }
                };
                current_names.remove(id);
                // match _photo {
                //     Some(photo) => println!("Removed photo: {:?}", photo),
                //     None => println!("Photo not found"),
                // }
            }
            2 => {
                let search_type = fastrand::u8(..=2);
                let id = match search_type {
                    0 => {
                        let start = std::time::Instant::now();
                        let _photo = gallery.get_photo(current_names[0], 0, FirstOrLast::First);
                        let end = start.elapsed().as_nanos();
                        search_first += end as f64;
                        search_first_counter += 1;
                        0
                    }
                    1 => {
                        let start = std::time::Instant::now();
                        let _photo = gallery.get_photo(
                            current_names[current_names.len() - 1],
                            current_names.len() - 1,
                            FirstOrLast::Last,
                        );
                        let end = start.elapsed().as_nanos();
                        search_last += end as f64;
                        search_last_counter += 1;
                        current_names.len() - 1
                    }
                    _ => {
                        let start = std::time::Instant::now();
                        let id = fastrand::usize(0..current_names.len());
                        let _photo = gallery.get_photo(current_names[id], id, FirstOrLast::Other);
                        let end = start.elapsed().as_nanos();
                        search_rand += end as f64;
                        search_rand_counter += 1;
                        id
                    }
                };
                current_names.remove(id);
                // match _photo {
                //     Some(photo) => println!("Found photo: {:?}", photo),
                //     None => println!("Photo not found"),
                // }
            }
            _ => unreachable!("Invalid operation"),
        }
    }
    println!(
        "{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6}",
        total.elapsed().as_nanos(),
        insert / insert_counter as f64,
        remove_first / remove_first_counter as f64,
        remove_last / remove_last_counter as f64,
        remove_rand / remove_rand_counter as f64,
        search_first / search_first_counter as f64,
        search_last / search_last_counter as f64,
        search_rand / search_rand_counter as f64
    );
}
