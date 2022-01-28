# Arrkia
Rei SDK and tools to create native GUI apps on QuantOS
Inspired by React components and React TSX. Components inspired by Chakra UI and Framer Motion
Syntax is scala-like and modules is based on sbt

## Features
- no semicolons, statements end in a newline character "\n" which is rendered as a newline in most text editors
- hook onto components and XML-like code with scala-like code to build complex components
- automatic modular system with directories, forces good use of structuring:
```dir
src/
    components/
        header.rei
        footer.rei
    index.rei
    pages/
        about.rei
        404.rei
    assets/
        visuals/
            Arrkia.gif
        fonts/
            Arrkia.ttf
        data/
            users.db
```
- builds for QuantOS native using ArcWM and Vulkano.
- does not build for browsers. We shouldnt be using the same language for two different things. If you want to build for the web, use JS, flutter or kotlin.

## Philosophy
- make things quick and straightfoward to make. Create now, optimise later. No need for types, but you can specify types if you want, e.g. in vital components where things shouldnt fail and global variables
- rely on underlying JIT and AOT compilers to validate and optimise the code
- TDD, built in testing with `arrkia test`. Includes negative testing and property based testing and integration testing, out of the box
- Common things, e.g. routing, components, animations etc. are built into Arrkia. More complex things and styled stuff are external libraries that can be installed locally and imported to your project files

## Boilerplate Code
When you type `arrkia new <name>` you create an arrkia project in `<name>/`. It then creates the following structure:
```dir
src/
    components/
        stock.rei
    index.rei
    pages/
        about.rei
    assets/
        visuals/
            Arrkia.gif #react-like spinning icon
        fonts/
            Arrkia.ttf #verdana
        data/
            users.db #mongodb
package.toml #specify dependencies for your final product and build dependencies, install packages with `arrkia i <package_name>` from arrkia.io
package.lock #locks your depenencies. Will not update your deps until you run `arrkia deps update`
config.rei #configuration options to edit your project name and useful scripts. `arrkia run` will build and run your app in hot-reload mode. `arrkia build --release` will build your app in `build/<name>.app` for QuantOS.
build.rei #use build dependencies and hook onto first and second stage of build process to insert scripts in rei
```
Note the users.db is for storing user data. Uses a key-value store based on mongodb. You can use your own DB by editing `config.rei`.

## FFX Code
So you have something like:

```ffx
// example.ffx file
import arrkia.ffx
import components.{Header, Footer}

val x: int = 5 // global variables must be typed. Also great for language servers
val __x: int = 6 // a variable private to this file

// all components are public by default
ComponentA({prop1, prop2}) = {
    {name, setName} = initState("")

    // a method is defined by the name and an equal-braces block
    handleClick() = {
        // ternary operators have first class support
        name == "Dixon" ? setName("Dixon") : ""
    }

    // the component's view are defined by the return statement
    ret (
        <>
            <Heading>This is ComponentA</Heading>
            <Body>
                <Box color='white' backgroundColor='black'>
                    Hi I am in a Box
                </Box>

                {prop1}

                <Flex flexDir='row'>
                    This is a flexbox
                </Flex>

                <Motion animate={[{shrink: 0.25, time: 0.5}, {expand:0.25, time:0.15}]}>
                    <Box>
                        <Text>This will SHRINK then EXPAND</Text>
                    </Box>
                </Motion>

                {prop2}

            </Body>
        </>
    )
}

// an _ underscore makes the component private to the module
_ComponentB() = {
    ret (
        <>
        </>
    )
}

// __ underscores makes the component private to the file
__ComponentC() = {
    ret (
        <>
        </>
    )
}

```

When you create a new Arrkia project, your index.ffx looks like:
```ffx
// boilerplate index.ffx
import arrkia.{ffx, Documentation}
import components.{Header, Footer}
import assets.visuals."Arrkia.gif" as ArrkiaSpinner

Main() = {
    ret (
        <>
            <Header>An Arrkia App</Header>
            <Body>
                <Motion src=ArrkiaSpinner/>
                <Documentation/>
            </Body>
            <Footer/>
        </>
    )
}

```

## Arrkia Creator
A QtCreator like IDE for Arrkia. Built using iced-rs.

