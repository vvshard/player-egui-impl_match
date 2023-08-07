This is an example of rust using the [methods_enum::`impl_match!`](https://github.com/vvshard/methods-enum) for GUI. (without player real connection)

![](/screenshot.png)

The idea of a player GUI as an example of rust's 'state' design pattern is taken [(here)](https://github.com/fadeevab/design-patterns-rust/tree/main/behavioral/state) with the addition of functionality, replacing console pseudo-graphics with [egui](https://github.com/emilk/egui/) via [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) and using `impl_match!` instead of dyn Trait.

Tested only on Windows 11, but eframe declares good cross-platform.
___
#### License
MIT