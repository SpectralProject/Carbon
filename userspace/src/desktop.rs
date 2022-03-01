// The desktop contains a layout of widgets on a grid. Each widget as a starting coordinate at its upper left. It grows down and right
// and its size is determined by w x h

type PairInt = u64;

struct Desktop {
    dimensions: PairInt,
}

struct Widget {
    starting_point: PairInt,
    dimensions: PairInt,
}

// WIDGETS CANNOT OVERLAP! I dont want to have to deal with overlapping widget logic
// but widgets can 'draw' ontop of other widget's space

// A window can be created and drawn ontop of the deskop at will
// we can do things like make the taskbar always ontop
