# Neutron Init

The init utility that is run from `sys/init.elf`. It does a lot of the config loading and stuff. Eventually, it runs a display manager which then runs the shell cli or the DE.
The DE can then itself run an app terminal emulator that uses the `console` program in `/sys/programs/console` . This program is built on `std` and can run any thing it wants.

Note programs in `sys/programs` are usually different to those in `/packages` as they are the defacto programs for a minimal functional system / CLI / programs that the use on a minimal hardware setup.
