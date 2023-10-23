# Rivet
 An asset for HardHat C2 written in Rust as an example/demo

## Installation and Use
1. Download cargo for the operating system the team server will be running on https://doc.rust-lang.org/cargo/getting-started/installation.html
2. Navigate to the main folder and execute ` dotnet build -c Release`
3. Copy the output Dlls to the `/HardHatC2/{HardHatC2Client or Teamserver}/Plugins` expected DLLs are `Rivet_ServerPlugin.dll` & `Rivet_ClientPlugin.dll`
4. If HardHat is already running, go to the Setting page and refresh the plugins
5. Build a test Rivet Asset from the build menu

### Use 
- Currently, Rivet is only meant to demo what a very basic Asset needs and should look like. It can run on Windows and has one command, which is just Whoami
