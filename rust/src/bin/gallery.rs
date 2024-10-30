use std::collections::{BTreeMap, HashMap, LinkedList};

#[derive(Debug)]
struct Photo {
    name: String,
}

trait Gallery {
    fn add_photo(&mut self, photo: Photo);
    fn remove_photo(&mut self, name: &str) -> Option<Photo>;
    fn get_photo(&self, name: &str) -> Option<&Photo>;
}

struct ArrayGallery {
    photos: Vec<Photo>,
}

struct LinkedListGallery {
    photos: LinkedList<Photo>,
}

struct HashGallery {
    photos: HashMap<String, Photo>,
}

struct BTreeGallery {
    photos: BTreeMap<String, Photo>,
}

impl Gallery for ArrayGallery {
    fn add_photo(&mut self, photo: Photo) {
        self.photos.push(photo);
    }

    fn remove_photo(&mut self, name: &str) -> Option<Photo> {
        let index = self.photos.iter().position(|x| x.name == name)?;
        Some(self.photos.remove(index))
    }

    fn get_photo(&self, name: &str) -> Option<&Photo> {
        let index = self.photos.iter().position(|x| x.name == name)?;
        Some(&self.photos[index])
    }
}

impl Gallery for LinkedListGallery {
    fn add_photo(&mut self, photo: Photo) {
        self.photos.push_back(photo);
    }

    fn remove_photo(&mut self, name: &str) -> Option<Photo> {
        let index = self.photos.iter().position(|x| x.name == name)?;
        let mut lists = self.photos.split_off(index);
        let photo = lists.pop_front()?;
        self.photos.append(&mut lists);
        Some(photo)
    }

    fn get_photo(&self, name: &str) -> Option<&Photo> {
        self.photos.iter().find(|x| x.name == name)
    }
}

impl Gallery for HashGallery {
    fn add_photo(&mut self, photo: Photo) {
        self.photos.insert(photo.name.clone(), photo);
    }

    fn remove_photo(&mut self, name: &str) -> Option<Photo> {
        self.photos.remove(name)
    }

    fn get_photo(&self, name: &str) -> Option<&Photo> {
        self.photos.get(name)
    }
}

impl Gallery for BTreeGallery {
    fn add_photo(&mut self, photo: Photo) {
        self.photos.insert(photo.name.clone(), photo);
    }

    fn remove_photo(&mut self, name: &str) -> Option<Photo> {
        self.photos.remove(name)
    }

    fn get_photo(&self, name: &str) -> Option<&Photo> {
        self.photos.get(name)
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let mut current_names = Vec::new();
    let mut insertion = 0.0;
    let mut removal = 0.0;
    let mut search = 0.0;

    let mut gallery: Box<dyn Gallery> = match args[1].as_str() {
        "array" => Box::new(ArrayGallery { photos: Vec::new() }),
        "hash" => Box::new(HashGallery {
            photos: HashMap::new(),
        }),
        "btree" => Box::new(BTreeGallery {
            photos: BTreeMap::new(),
        }),
        "linked" => Box::new(LinkedListGallery {
            photos: LinkedList::new(),
        }),
        _ => panic!("Invalid gallery type"),
    };
    let n = args[2]
        .parse::<usize>()
        .expect("Invalid number of operations to execute");

    let seed = args[3]
        .parse::<u64>()
        .expect("Invalid seed for random number generator");
    fastrand::seed(seed);
    let total = std::time::Instant::now();
    for n in 0..n {
        let op = if current_names.is_empty() {
            0
        } else {
            fastrand::u8(..=2)
        };

        match op {
            0 => {
                let start = std::time::Instant::now();
                let photo = Photo {
                    name: format!("photo_{}", n),
                };
                gallery.add_photo(photo);
                current_names.push(format!("photo_{}", n));
                let end = start.elapsed().as_nanos();
                insertion += end as f64;
            }
            1 => {
                let id = fastrand::usize(0..current_names.len());
                let start = std::time::Instant::now();
                let _photo = gallery.remove_photo(current_names[id].as_str());
                let end = start.elapsed().as_nanos();
                removal += end as f64;
                current_names.remove(id);
                // match _photo {
                //     Some(photo) => println!("Removed photo: {:?}", photo),
                //     None => println!("Photo not found"),
                // }
            }
            2 => {
                let id = fastrand::usize(0..current_names.len());
                let start = std::time::Instant::now();
                let _photo = gallery.get_photo(current_names[id].as_str());
                let end = start.elapsed().as_nanos();
                search += end as f64;
                // match _photo {
                //     Some(photo) => println!("Found photo: {:?}", photo),
                //     None => println!("Photo not found"),
                // }
            }
            _ => unreachable!("Invalid operation"),
        }
    }
    println!("Total time: {:?}", total.elapsed());
    println!(
        "Insertion: {:.2}ns, Removal: {:.2}ns, Search: {:.2}ns",
        insertion / 1000.0,
        removal / 1000.0,
        search / 1000.0
    );
}
