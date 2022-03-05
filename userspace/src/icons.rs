//
// read from bootstrap-icons/*.svg into Vec<String>
// for each String, std::read() them and assign a key: val <string: SVG_Data>
// now you should be able to index like HashMap["arrow-bar-left"] to reference the svg data

// then when you want to use it to render something on a context, e.g. a window or vulkan context
// use it as a texture

// FFX:
// <Img svg={quanta::<icon_name>}/>

/*

return FFX`
<Box>
    <Heading>This is a heading</Heading>
</Box>
`
*/

// Rei Language server with sockets
// Open an RPC socket. Bind the address/pid of the running RPC server on localhost:<port>
// get the socket_descriptor of the RPC socket and send/receive JSON strings through reads/writes to the socket
// would usually sit in a constant loop waiting for a queued request and accept()
// if a change detected in your code, would make a send()

// Rei LSP with gRPC
// request-reply on UDP for lower overhead, esp on local machines where errors are much rarer
// Start the Language server reiserver on <pid>
// In the IDE, open a channel communication pipe with the process directly (IPC) -> pipe_open(pid)
// 

