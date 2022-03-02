// PACKAGE MANAGER

use crate::App;

// kinda like functional app managers/

// SYMLINK Command => symlink <src> <dst>

// A package is a more generic view of an app
// A package may contain 1 or more executables and static/dynamic libraries
// which can be symlinked to the default place /bin/ and /lib/
// NOTE: no '/usr' or '/local'
struct Package {
    name: String,
    exes: Vec<Executable>,
    staticlibs: Vec<StaticLibrary>,
    dynamiclibs: Vec<DynamicLibrary>,
    apps: Vec<App>,
}

// needs to be .elf on neutron
struct Executable {
    path: String,
}
// should be .a, .lib or .o. Shouldnt be .o usually
struct StaticLibrary {
    path: String,
}
// usually .so
struct DynamicLibrary {
    path: String,
}

pub trait AppInstall {
    fn install_app(&self);
}

impl Package {
    pub fn new(_name: &str) -> Self {
        Self {
            name: _name.to_string(),
            exes: vec![],
            staticlibs: vec![],
            dynamiclibs: vec![],
            apps: vec![],
        }
    }
    // a package should automatically link any libraries or executables to /bin and /lib
    // by using the List<Executable> and List<StaticLibrary> interfaces
    // The executable searcher searches /bin for executables
    // But for apps, we search /app. Apps can be installed by packages too via the AppInstall interface
    pub fn install_executables(&mut self) {
        // assumes we have access to the main filesystem (default filesystem) at a root level
        // which should be the case for standard users. But maybe not for guest containers

        // executables.map(e -> ...)
        self.exes.iter().for_each(|p| {
            let command = "
            #!/bin/reis
            
            symlink {p.path} /bin
            ";

            todo!("create a reis command with std::command")
        });

        self.staticlibs.iter().for_each(|p| {
            let command = std::format!(
                "
            #!/bin/reis
            
            symlink {} /lib/static
            ",
                p.path
            );
        });

        self.dynamiclibs.iter().for_each(|p| {
            let command = std::format!(
                "
            #!/bin/reis
            
            symlink {} /lib/shared
            ",
                p.path
            );
        });
    }
}

impl AppInstall for Package {
    fn install_app(&self) {
        self.apps.iter().for_each(|a| {
            let command = std::format!(
                "
            #!/bin/reis
            
            symlink {} /lib/shared
            ",
                a.executable_path
            );
        });
    }
}

struct PackageManager {
    packages: Vec<Package>,
}

impl PackageManager {
    pub fn new() -> Self {
        Self { packages: vec![] }
    }

    pub fn add_package(&mut self, package: Package) {
        // Packages should be namespaced like <namespace>/<package> so we dont need package ids
        // except maybe md5 checksums to check if its legit
        match self.packages.iter().find(|p| p.name == package.name) {
            Some(p) => {
                println!(
                    "Package {} already exists! Did you mean to update or uninstall it?",
                    p.name
                );
            }
            None => {
                println!("Installing package {}...", package.name);
                // need async fetch code
                // TODO: check MD5 checksum after fetching from neutron-packages.io
                let md5_checksum = ();
                let official_checksum = ();
                assert_eq!(md5_checksum, official_checksum);

                self.packages.push(package);
            }
        }
    }

    pub fn remove_package(&mut self, package_name: &str) {
        let res = self.packages.retain(|p| p.name != package_name);
    }
}

// WHAT I DONT WANT:
// Windows "package managers"
// stuff like chocolatey, or worse, winget
// choco gui is decent enough but nowhere as good as gnome frontends or even kde frontends

// Like the android app store, windows store or apple ios store
// popular and featured rows
// editor's choice
struct Frontend;
