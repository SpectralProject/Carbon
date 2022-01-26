extern crate neutron_wm;

fn main() {
    // open up a window
    let win = neutron_wm::create_window();
    // use the compiled the layout XML like how Qt does it. The default XML starts off at the home page with a custom google search, toolbars and titlebar
    // NOTE: an async function. While loading, load the other stuff like user profiles, settings, themes
    win.use_XML("../res/default_layout.xml");
    
    // load browser settings, home page, bookmarks and populate toolbars, extensions
    // ctrl -> file, edit, help basically. Should be hidden though by the XML until you press ctrl
    win.set("toolbarCtrl", default_toolbar_ctrl_setting);
    win.set("toolbarTabs", default_toolbar_tabs_setting);
    win.set("toolbarSearch", default_toolbar_search_setting);
    win.set("toolbarBookmarks", default_toolbar_bookmarks_setting)

    // other things like the settings page, about:config should be loaded in the background? Or are they loaded automatically by the XML
    // maybe the functionality might not have to be automatically loaded but instead dynamically linked but idk
    // would result in faster boottime and not many people access settings anyway
    // NOTE: load is an async function
    // arg: XML file defining functionality or DLL with the functionality
    win.load(settings_page_functionality);
    win.load(config_page_functionality);
}

// maybe stuff about caching

// security

// convenience, cookies, password management, history, bookmarks

// special pages like 404, 500

// ways to render and display HTML and CSS, prob the biggest thing
// can use a renderer maybe

// dev tools
