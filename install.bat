@echo off
REM Dual Engine - Global Installation Script
REM Run this to install dual-engine globally on Windows

set "SCRIPT_DIR=%~dp0"
set "PROJECT_DIR=%SCRIPT_DIR%.."

echo ========================================
echo Dual Engine - Global Installation
echo ========================================
echo.

set "USER_BIN=%USERPROFILE%\bin"
if not exist "%USER_BIN%" mkdir "%USER_BIN%"

echo Copying binaries to %USER_BIN%...
copy /Y "%PROJECT_DIR%\bin\de.exe" "%USER_BIN%\de.exe"
copy /Y "%PROJECT_DIR%\bin\det.exe" "%USER_BIN%\det.exe"
copy /Y "%PROJECT_DIR%\bin\opencode.exe" "%USER_BIN%\opencode.exe"
copy /Y "%PROJECT_DIR%\bin\claude.exe" "%USER_BIN%\claude.exe"

echo.
echo Setting environment variables...
setx OPENCODE_BIN "%USER_BIN%\opencode.exe" >nul 2>&1
setx CLAUDE_BIN "%USER_BIN%\claude.exe" >nul 2>&1
setx PATH "%USER_BIN%;%PATH%" >nul 2>&1

echo.
echo ========================================
echo Installation complete!
echo.
echo Commands:
echo   de    - CLI mode (dual-engine)
echo   det   - TUI mode (dual-engine-tui)
echo.
echo Binaries copied to: %USER_BIN%
echo.
echo Please restart your terminal or run:
echo   refreshenv
echo ========================================
pause