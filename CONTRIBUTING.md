# 🤝 Contributing to RustyConfig

Thank you for your interest in RustyConfig! We welcome all contributions that improve the project.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How can I contribute?](#how-can-i-contribute)
- [Development Environment Setup](#development-environment-setup)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Tests](#tests)
- [Documentation](#documentation)
- [Release Process](#release-process)

## 📜 Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## 🚀 How can I contribute?

### Types of Contributions

- 🐛 **Bug Reports**: Report bugs and issues
- ✨ **Feature Requests**: Suggest new features
- 🔧 **Code Contributions**: Implement features or fixes
- 📚 **Documentation**: Improve documentation
- 🧪 **Tests**: Extend test coverage
- 🌍 **Translations**: Translate documentation

### Getting Started

1. Check out the [Issues](https://github.com/jakubbbdev/rusty-config/issues)
2. Look for issues labeled "good first issue" or "help wanted"
3. Comment on an issue that you'd like to work on
4. Fork the repository and create a feature branch

## 🛠️ Development Environment Setup

### Prerequisites

- Rust 1.70 or higher
- Cargo
- Git

### Setup

```bash
# Clone repository
git clone https://github.com/jakubbbdev/rusty-config.git
cd rusty-config

# Install dependencies
cargo build

# Run tests
cargo test

# Linting
cargo clippy

# Formatting
cargo fmt
```

### Useful Tools

```bash
# Install Rustup components
rustup component add rustfmt
rustup component add clippy
rustup component add rust-docs

# Install Cargo tools
cargo install cargo-audit
cargo install cargo-tarpaulin  # for code coverage
cargo install cargo-watch       # for auto-restart
```

## 🔄 Pull Request Process

### 1. Create Branch

```bash
git checkout -b feature/amazing-feature
# or
git checkout -b fix/bug-description
```

### 2. Develop Changes

- Write clean, documented code
- Add tests for new features
- Ensure all tests pass
- Update documentation

### 3. Create Commits

```bash
# Stage changes
git add .

# Create commit with descriptive message
git commit -m "feat: add amazing new feature"

# Push to your fork
git push origin feature/amazing-feature
```

### 4. Submit Pull Request

- Go to [GitHub](https://github.com/jakubbbdev/rusty-config)
- Click "New Pull Request"
- Select your branch
- Fill out the PR template
- Submit for review

## 📝 Coding Standards

### Rust Style Guide

- Follow [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Maximum line length: 100 characters

### Code Documentation

```rust
/// Brief description of the function
///
/// # Arguments
/// * `param` - Description of parameter
///
/// # Returns
/// Description of return value
///
/// # Examples
/// ```
/// use rusty_config::some_function;
/// let result = some_function("example");
/// ```
pub fn some_function(param: &str) -> Result<String, Error> {
    // Implementation
}
```

### Error Handling

- Use `Result<T, E>` for fallible operations
- Provide meaningful error messages
- Use `thiserror` for custom error types
- Handle errors gracefully

## 🧪 Tests

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Arrange
        let input = "test";
        
        // Act
        let result = function(input);
        
        // Assert
        assert_eq!(result, expected);
    }

    #[tokio::test]
    async fn test_async_function() {
        // Async test implementation
    }
}
```

### Test Guidelines

- Write unit tests for all public functions
- Use descriptive test names
- Follow AAA pattern (Arrange, Act, Assert)
- Test both success and failure cases
- Use `#[should_panic]` for panic tests
- Use `#[ignore]` for slow tests

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run ignored tests
cargo test -- --ignored

# Run tests with coverage
cargo tarpaulin
```

## 📚 Documentation

### Code Documentation

- Document all public APIs
- Use doc comments (`///`)
- Include examples in documentation
- Keep documentation up to date

### README Updates

- Update README.md for new features
- Add examples for new functionality
- Update installation instructions
- Keep feature list current

### API Documentation

```bash
# Generate documentation
cargo doc

# Open documentation in browser
cargo doc --open

# Generate documentation for all features
cargo doc --all-features
```

## 🚀 Release Process

### Version Bumping

- Follow [Semantic Versioning](https://semver.org/)
- Update version in `Cargo.toml`
- Update `CHANGELOG.md`
- Create release notes

### Pre-release Checklist

- [ ] All tests pass
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated
- [ ] Version is bumped
- [ ] Code is formatted
- [ ] Linting passes
- [ ] Security audit passes

### Release Steps

1. Create release branch
2. Update version numbers
3. Update CHANGELOG.md
4. Create pull request
5. Get review and approval
6. Merge to main
7. Create GitHub release
8. Publish to crates.io

## 🎯 Issue Templates

### Bug Report Template

```markdown
## Bug Description
Brief description of the bug

## Steps to Reproduce
1. Step 1
2. Step 2
3. Step 3

## Expected Behavior
What should happen

## Actual Behavior
What actually happens

## Environment
- OS: [e.g., Windows 10, macOS 12.0]
- Rust Version: [e.g., 1.70.0]
- Crate Version: [e.g., 0.1.0]

## Additional Information
Any other relevant information
```

### Feature Request Template

```markdown
## Feature Description
Brief description of the feature

## Use Case
Why is this feature needed?

## Proposed Solution
How should this feature work?

## Alternatives Considered
Other approaches that were considered

## Additional Information
Any other relevant information
```

## 📞 Getting Help

- Create an [Issue](https://github.com/jakubbbdev/rusty-config/issues)
- Join our [Discussions](https://github.com/jakubbbdev/rusty-config/discussions)
- Check the [Documentation](https://docs.rs/rusty-config)

## 🙏 Recognition

Contributors will be recognized in:
- README.md contributors section
- Release notes
- GitHub contributors page

Thank you for contributing to RustyConfig! 🦀 