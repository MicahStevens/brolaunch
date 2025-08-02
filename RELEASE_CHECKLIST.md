# Release Checklist

Follow these steps when creating a new release:

## Pre-release

1. **Update version number**
   ```bash
   # Edit Cargo.toml version field
   # Example: version = "0.2.0"
   ```

2. **Update CHANGELOG.md**
   - Add new version section with date
   - List all changes since last release
   - Follow Keep a Changelog format

3. **Test thoroughly**
   ```bash
   cargo test
   cargo build --release
   ./target/release/brolaunch --version
   ```

4. **Commit changes**
   ```bash
   git add Cargo.toml CHANGELOG.md
   git commit -m "Release v0.2.0"
   ```

## Create Release

1. **Create and push tag**
   ```bash
   git tag v0.2.0
   git push origin main
   git push origin v0.2.0
   ```

2. **Wait for GitHub Actions**
   - The release workflow will automatically:
     - Build Linux x86_64 binary
     - Create GitHub release
     - Upload binary artifacts
     - Extract changelog for release notes

3. **Verify release**
   - Check https://github.com/MicahStevens/brolaunch/releases
   - Download and test the binary
   - Ensure changelog is properly displayed

## Post-release

1. **Update README if needed**
   - Update installation instructions with new version
   - Add any new features to feature list

2. **Announce release** (optional)
   - Share in relevant communities
   - Update any documentation sites

## Version Numbering

Follow Semantic Versioning:
- MAJOR.MINOR.PATCH (e.g., 1.2.3)
- MAJOR: Breaking changes
- MINOR: New features (backwards compatible)
- PATCH: Bug fixes (backwards compatible)