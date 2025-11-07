@echo off
echo ========================================
echo Dioxus Migration - Automated Setup
echo ========================================
echo.
echo This script will convert your Tauri app to Dioxus Desktop
echo.
echo What it does:
echo   1. Backup your current files
echo   2. Copy new Dioxus files into place
echo   3. Update Cargo.toml
echo   4. Create components directory
echo.

pause

echo.
echo Step 1: Creating backup...
if not exist "backup" mkdir backup
copy Cargo.toml backup\Cargo.toml.bak
copy src\main.rs backup\main.rs.bak
echo Backup created in 'backup' folder
echo.

echo Step 2: Updating Cargo.toml...
copy dioxus-migration\new_Cargo.toml Cargo.toml
echo Cargo.toml updated
echo.

echo Step 3: Updating main.rs...
copy dioxus-migration\new_main.rs src\main.rs
echo main.rs updated
echo.

echo Step 4: Creating app.rs...
copy dioxus-migration\app.rs src\app.rs
echo app.rs created
echo.

echo Step 5: Creating components directory...
if not exist "src\components" mkdir src\components
copy dioxus-migration\natal.rs src\components\natal.rs
copy dioxus-migration\synastry_complete.rs src\components\synastry.rs
copy dioxus-migration\transits_complete.rs src\components\transits.rs
copy dioxus-migration\mod.rs src\components\mod.rs
echo Components created
echo.

echo ========================================
echo Migration Complete!
echo ========================================
echo.
echo Next steps:
echo   1. Run: cargo build --release
echo   2. Test: cargo run --release
echo.
echo If anything goes wrong:
echo   - Restore files from 'backup' folder
echo   - Or refer to INSTALLATION_GUIDE.md
echo.
echo Your calculation code (chart.rs, sweph.rs, etc.) is unchanged!
echo.

pause
