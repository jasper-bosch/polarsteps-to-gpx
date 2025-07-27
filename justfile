default:
    just --list

# Install the project as a binary in the cargo bin directory
install:
    cargo install --path .

################################################################################

# Build the project in release mode
[group('build')]
build:
    cargo build --release

# Clean the project
[group('build')]
clean:
    cargo clean

# Build the project in debug mode
[group('build')]
debug:
    cargo build

# Clean and then build the project in release mode
[group('build')]
rebuild: clean build

################################################################################

# Check the project for formatting, linting, and tests (requires cargo-machete and -outdated)
[group('check')]
check:
    cargo fmt -- --check
    cargo clippy -- -D warnings
    cargo test
    cargo machete
    cargo outdated -R

# Check for unused/outdated/unmaintained dependencies (requires cargo-machete, -outdated and -unmaintained)
[group('check')]
deps:
    cargo machete
    cargo outdated -R --exit-code 1
    cargo unmaintained

# Run clippy on the project and treat warnings as errors
[group('check')]
lint:
    cargo clippy -- -D warnings

################################################################################

# Generate and open the documentation
[group('dev')]
docs:
    cargo doc --open

# Automatically fix lint warnings
[group('dev')]
fix:
    cargo fix --allow-dirty

# Install development tools
[group('dev')]
init:
    cargo install cargo-edit
    cargo install cargo-machete
    cargo install cargo-outdated
    cargo install cargo-unmaintained

# Run the tests
[group('dev')]
test:
    cargo test

# Upgrade dependencies (requires cargo-edit)
[group('dev')]
upgrade:
    cargo upgrade

################################################################################

# Bump the major version (requires cargo-edit)
[group('version')]
bump-major:
    cargo set-version --bump major

# Bump the minor version (requires cargo-edit)
[group('version')]
bump-minor:
    cargo set-version --bump minor

# Bump the patch version (requires cargo-edit)
[group('version')]
bump-patch:
    cargo set-version --bump patch
