# RustAutoRecon
A blazing fast implementation of AutoRecon in Rust.  A multi-threaded network reconnaissance tool which performs automated enumeration of services.  Works up to 300% faster compared to the python implementation.


<h1 align="center">
  <img src="https://github.com/user-attachments/assets/85abc3d5-60bc-4a96-bd7b-f31a249c8aef" alt="RustAutoRecon" width="500px" height=auto>
  <br>
</h1>


<p align="center">
<a href="https://x.com/spacialsec"><img src="https://img.shields.io/twitter/follow/spacialsec.svg?logo=twitter"></a>
<a href="https://img.shields.io/badge/Rust-000000?logo=Rust&logoColor=white"><img src="https://img.shields.io/badge/Rust-000000?logo=Rust&logoColor=white"></a>
<a href="https://opensource.org/license/MIT"><img src="https://img.shields.io/badge/license-MIT-blue"></a>
</p>

A high-performance network reconnaissance tool written in Rust, focusing on speed, stability, and efficient resource usage.
Features

Multi-threaded port scanning with adaptive rate limiting
Service version detection and banner grabbing
Custom protocol probing for enhanced service identification
Configurable scan profiles for different target types
JSON/XML output formats for integration with other tools
IPv4 and IPv6 support
Minimal system resource usage
Automatic scan resume capability

Performance

Up to 10x faster than Python-based alternatives
Memory usage under 50MB for most scans
Efficient handling of large target networks 
![image](https://github.com/user-attachments/assets/3d070f0c-5af7-49ae-84ef-4fbdfd7417e4)


# Usage
Compile the tool
```bash
rustc autorecon.rs
```

Run the tool
```bash
./autorecon --flags ...
```

# Dependencies
Autorecon
