# Quick Start - Build with GitHub Actions

## Step 1: Initialize Git Repository (if not already done)

```bash
git init
git add .
git commit -m "Initial commit - taskhomie"
```

## Step 2: Create GitHub Repository

1. Go to https://github.com/new
2. Create a new repository (public or private)
3. Name it `taskhomie` (or your preferred name)
4. **Don't** initialize it with README, .gitignore, or license

## Step 3: Push to GitHub

```bash
# Replace YOUR_USERNAME with your GitHub username
git remote add origin https://github.com/YOUR_USERNAME/taskhomie.git
git branch -M main
git push -u origin main
```

## Step 4: Enable GitHub Actions

1. Go to your repository on GitHub
2. Click on the **"Actions"** tab
3. If prompted, click **"I understand my workflows, go ahead and enable them"**

## Step 5: Trigger the Build

### Option A: Automatic Build
The workflow will automatically start when you push to `main` branch.

### Option B: Manual Build
1. Go to **Actions** tab
2. Click on **"Build Tauri App"** workflow
3. Click **"Run workflow"**
4. Select branch (main)
5. Click **"Run workflow"**

## Step 6: Download Your Build

Wait for the workflow to complete (20-40 minutes for first build):

1. Click on the completed workflow run (green checkmark)
2. Scroll down to **"Artifacts"** section
3. Click on **`taskhomie-windows-installer`** (or your platform)
4. Install the downloaded `.msi` file

## That's It! 🎉

You now have a fully built Taskhomie application without installing Visual Studio!

---

## Creating a Release (Optional)

To create a release with all installers attached:

```bash
# Create a version tag
git tag v0.1.0
git push origin v0.1.0
```

The workflow will automatically:
- Build for all platforms
- Create a draft release
- Attach all installers

Go to **Releases** on GitHub to review and publish!

---

## Troubleshooting

### Workflow Doesn't Start?
- Check if Actions are enabled in repo settings
- Check if you have permission to run workflows

### Build Fails?
- Click on the failed job to see logs
- Common issues:
  - Missing icon files (we already fixed this)
  - Rust compilation errors
  - Missing dependencies

### Need to Rebuild?
- Just push a new commit
- Or use "Run workflow" button for manual build

---

**Need help?** See `GITHUB_ACTIONS_GUIDE.md` for detailed documentation.
