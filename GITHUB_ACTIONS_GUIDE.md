# GitHub Actions Build Guide

## Overview

This workflow builds Taskhomie for Windows, macOS, and Linux in the cloud using GitHub Actions. No need to install Visual Studio or other build tools locally!

## How to Use

### Option 1: Automatic Build on Push

1. **Push your code to GitHub:**
   ```bash
   git init
   git add .
   git commit -m "Initial commit"
   git remote add origin https://github.com/YOUR_USERNAME/taskhomie.git
   git push -u origin main
   ```

2. **Go to GitHub Actions:**
   - Navigate to your repository on GitHub
   - Click on the "Actions" tab
   - You'll see the "Build Tauri App" workflow running automatically

3. **Download the build artifacts:**
   - Once the workflow completes (green checkmark)
   - Click on the workflow run
   - Scroll down to "Artifacts"
   - Click on the artifact for your platform:
     - `taskhomie-windows-installer` - Windows MSI installer
     - `taskhomie-windows-exe-installer` - Windows EXE installer (NSIS)
     - `taskhomie-macos-dmg` - macOS DMG
     - `taskhomie-linux-deb` - Linux Debian package
     - `taskhomie-linux-appimage` - Linux AppImage

### Option 2: Manual Build (Workflow Dispatch)

1. **Go to Actions tab**
2. **Select "Build Tauri App" workflow**
3. **Click "Run workflow"**
4. **Select the branch** you want to build
5. **Click "Run workflow"**

### Option 3: Create a Release (with Auto-generated Release Notes)

1. **Create a version tag:**
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

2. **The workflow will automatically:**
   - Build for all platforms
   - Create a draft GitHub release
   - Attach all installers to the release

3. **Go to Releases** on GitHub to review and publish the draft release

## Build Outputs

### Windows
- **MSI Installer**: `taskhomie_0.1.0_x64.msi` - Standard Windows installer
- **EXE Installer**: `taskhomie_0.1.0_x64-setup.exe` - NSIS installer (if configured)
- **Standalone EXE**: `taskhomie.exe` - Portable executable

### macOS
- **DMG**: `taskhomie_0.1.0_x64.dmg` - Disk image with app
- **App Bundle**: `taskhomie.app` - Application bundle

### Linux
- **DEB**: `taskhomie_0.1.0_amd64.deb` - Debian/Ubuntu package
- **AppImage**: `taskhomie_0.1.0_amd64.AppImage` - Universal Linux package
- **RPM**: `taskhomie_0.1.0.x86_64.rpm` - Red Hat/Fedora package

## Workflow Features

✅ **Multi-platform builds** - Windows, macOS, and Linux  
✅ **Automatic artifact upload** - Download ready-to-install packages  
✅ **Release automation** - Create releases with all installers attached  
✅ **Build caching** - Faster subsequent builds  
✅ **Pull request builds** - Verify changes before merging  

## Troubleshooting

### Build Fails

1. **Check the workflow logs** - Click on the failed job to see detailed error messages
2. **Common issues:**
   - Missing dependencies (check `package.json`)
   - Rust compilation errors (check `src-tauri/src/`)
   - Icon files missing (ensure `icons/` folder has all required files)

### Artifacts Expired

GitHub keeps artifacts for 90 days by default. If expired:
- Re-run the workflow to generate new artifacts
- Or create a release to permanently attach binaries

### Build Takes Too Long

First build takes ~30-40 minutes (compiling all Rust dependencies). Subsequent builds are faster (~10-15 minutes) due to caching.

## Customization

### Change Node Version

Edit `.github/workflows/build.yml`:
```yaml
- name: Setup Node.js
  uses: actions/setup-node@v4
  with:
    node-version: '18'  # Change to your preferred version
```

### Add API Keys for Features

If your app needs API keys (e.g., for analytics):
```yaml
- name: Build Tauri App
  run: npm run tauri build
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
    YOUR_API_KEY: ${{ secrets.YOUR_API_KEY }}
```

Add secrets in: GitHub Repo → Settings → Secrets and variables → Actions

### Build Only Specific Platform

Comment out the jobs you don't need in `build.yml`.

## Cost

GitHub Actions is **free** for public repositories.

For private repositories:
- 2,000 minutes/month free
- Windows builds use ~20-30 minutes
- macOS builds use ~15-20 minutes
- Linux builds use ~10-15 minutes

## Next Steps

1. ✅ Push code to GitHub
2. ✅ Enable Actions (if not already enabled)
3. ✅ Run the workflow
4. ✅ Download your built app!

---

**Need help?** Check the [GitHub Actions documentation](https://docs.github.com/en/actions) or open an issue.
