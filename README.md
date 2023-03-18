# Heligman-Pollard 8d Visualizer

Allows you to edit HP mortality parameters in real time with updating visualizations. Heavy caclulations and rendering are offloaded into Rust to vastly improve performance, and allow smooth exploration of the 8th dimensional space of an HP equation.

## Building
Requires a current version of Rust and .NET installed. The DLL in `heligman-pollard-image-gen` should be built with `cargo build -r` and then
copied to the C# project, overwriting `Heligman-Pollard-Visualizer/heligman_pollard_image_gen.dll`. The C# project can be built normally in Visual Studio.

## Running
The release files only require an up-to-date .NET runtime installed on your machine. They can be found here: https://github.com/OtterCode/heligman-pollard-8d-visualizer/releases/tag/1.0.0
