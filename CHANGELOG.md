# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of RustyConfig
- Multi-format configuration support (JSON, YAML, TOML)
- Hot-reload functionality with file watching
- Validation framework with type validators
- Builder pattern for easy configuration creation
- Async/await support throughout the API
- Comprehensive error handling
- Version tracking for configuration changes
- Support for conditional compilation with features

### Features
- `yaml` - YAML format support (default enabled)
- `json` - JSON format support (default enabled)
- `toml` - TOML format support (default enabled)
- `hot-reload` - Hot-reload functionality
- `validation` - Validation framework
- `logging` - Logging integration

## [0.1.0] - 2025-07-20

### Added
- Core configuration management library
- ConfigBuilder with fluent API
- File format auto-detection
- Type-safe configuration handling
- Comprehensive documentation and examples
- GitHub Actions CI/CD pipeline
- MIT License

### Technical Details
- Built with Rust 1.70+
- Uses Tokio for async runtime
- Serde for serialization/deserialization
- Notify for file watching
- Thiserror for error handling
- UUID for watcher identification 