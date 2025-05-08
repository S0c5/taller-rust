# Multi-Platform Library Reuse Workshop

This workshop demonstrates how to create and reuse a library across multiple platforms and programming languages. The project showcases best practices for cross-platform development, testing, and distribution.

## Project Structure

```
.
├── core/                  # Core library implementation
│   ├── src/              # Source code
│   ├── tests/            # Unit tests
│   └── build/            # Build artifacts
├── platforms/            # Platform-specific implementations
│   ├── python/          # Python bindings and examples
│   ├── javascript/      # JavaScript/Node.js bindings and examples
│   └── rust/            # Rust bindings and examples
├── docs/                # Documentation
├── examples/            # Cross-platform usage examples
└── scripts/             # Build and deployment scripts
```

## Prerequisites

- Python 3.8+
- Node.js 16+
- Rust 1.70+
- CMake 3.20+
- Git
- wasm-pack (install via `cargo install wasm-pack`)

## Building the Project

### Core Library

1. Navigate to the core directory:
   ```bash
   cd core
   ```

2. Create a build directory:
   ```bash
   mkdir build && cd build
   ```

3. Configure and build:
   ```bash
   cmake ..
   make
   ```

### Platform-Specific Builds

#### Python
```bash
cd platforms/python
pip install -e .
```

#### JavaScript
```bash
cd platforms/javascript
npm install
npm run build
```

#### Rust
```bash
cd platforms/rust
cargo build
```

## Testing

### Core Library Tests
```bash
cd core/build
ctest
```

### Platform-Specific Tests

#### Python
```bash
cd platforms/python
pytest
```

#### JavaScript
```bash
cd platforms/javascript
npm test
```

#### Rust
```bash
cd platforms/rust
cargo test
```

## Release Process

1. Update version numbers in all platform-specific package files
2. Run all tests across platforms
3. Build release artifacts:
   ```bash
   ./scripts/build-release.sh
   ```
4. Create GitHub release with release notes
5. Publish packages to respective registries:
   - Python: PyPI
   - JavaScript: npm
   - Rust: crates.io

## Development Workflow

1. Create a new branch for your feature
2. Implement changes in the core library
3. Update platform-specific bindings
4. Add tests for new functionality
5. Run all tests across platforms
6. Create pull request

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Workshop Agenda

1. Introduction to cross-platform development
2. Core library implementation
3. Creating language bindings
4. Testing strategies
5. Build and release process
6. Hands-on exercises

## Resources

- [Documentation](docs/)
- [API Reference](docs/api.md)
- [Examples](examples/)
- [Contributing Guidelines](CONTRIBUTING.md)
