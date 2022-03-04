//
// read from bootstrap-icons/*.svg into Vec<String>
// for each String, std::read() them and assign a key: val <string: SVG_Data>
// now you should be able to index like HashMap["arrow-bar-left"] to reference the svg data

// then when you want to use it to render something on a context, e.g. a window or vulkan context
// use it as a texture

// FFX:
// <Img svg={quanta::<icon_name>}/>
