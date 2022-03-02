// functions to use the window API and etc.
// default userspace package (Users, Desktop Environment and Layout, Dock, Search functions, Settings)

// should be used on the HFS not Semantic FS
type AssetPath = String;

// semantic filesystem implemented in userspace instead of kernel space
struct SemanticFS;

// for userspace configuration on a per-user basis
// NO: admin-guest split on the default usermode. If you want to simulate a guest, use a guest container instead
struct UserspaceSettings {
    wallpaper: AssetPath,
    user_semantic_fs: SemanticFS,
}


// usually to do with graphical interfaces like app docks/drawers. Packages are more to do with the backend view
// an app, representation of a single program (usually installed as a single executable with supporting assets)
// a single icon or animation as its thumbnail
// can be searched in the search dock
pub struct App {
    executable_path: AssetPath,
    thumbnail: AssetPath,
}

impl App {
    pub fn new() -> Self {
        Self {
            executable_path: "/app".to_string(),
            thumbnail: "/thumbnail".to_string(),
        }
    }
}

// searcher for apps and userspace settings
// for kernel config, use KernelSettings that requires key-val editing like windows HKEY
// should have an index of all files on all disks
// FOR NOW: do for main disk withdefault settings, i.e. a system partition, recovery partition, and main filesystem
struct Searcher {
    // (disk_number, partition_number), assumes using main HFS formatting
    main_filesystem: (u64, u64),
}

impl Searcher {
    // should reindex in the background every 30 min
    pub fn index_fs(&self) {
        // get kernel to make db for you
    }
}

// LIBRARIES

pub mod desktop;
pub mod package;
