# Heligman-Pollard 8d Visualizer

Allows you to edit HP mortality parameters in real time with updating visualizations.

## Building
Requires a current version of Rust and .NET installed. The DLL in `heligman-pollard-image-gen` should be built with `cargo build -r` and then
copied to the C# project, overwriting `Heligman-Pollard-Visualizer/heligman_pollard_image_gen.dll`. The C# project can be built normally in Visual Studio.
