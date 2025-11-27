## Local Tooling Notes (Auto-Generated)

- Node.js and npm are required to run project scripts. Install Node.js LTS from https://nodejs.org/en and ensure 'Add to PATH' is selected in the installer.
- On Windows, the winget CLI depends on the 'App Installer' package from Microsoft Store and typically requires admin rights to install or repair.
- Scripts added:
  - scripts/Inspect-Wasm.ps1 – checks a file for the phrase "wasm-objdump" without modifying it.
  - scripts/AutoFix-Npm.ps1 – detects missing node/npm/winget and prints safe installation guidance.
  - scripts/GitHub-Platform-Improvements.ps1 – configures convenient git settings and aliases with no long-running loops.

These scripts are provided as-is, with the intention of avoiding harmful behavior and respecting user permissions.
# Github-Solutions Repository

ALN (Advanced Language Notation) governance, CI workflows, and supporting Node + PowerShell tooling.

## Key Directories / Files
- `aln/` : Source ALN bundles and governance documents.
- `aln-json/` : Generated JSON projections (created by scripts).
- `schemas/` : JSON Schemas used by Ajv mesh sweep.
- `scripts/` : Node and PowerShell helper scripts.
  - `aln-to-json-projection.cjs` : Converts `.aln` → JSON projection.
  - `aln-ajv-mesh-sweep.cjs` : Runs Ajv validation across projections.
  - `aln-severity-gate.cjs` : Evaluates sevmesh-gate thresholds.
  - `aln-copilot-metatest.cjs` : Validates Copilot metaprompt governance.
  - `Inspect-Wasm.ps1` : WASM inspection helper (references `wasm-objdump`).
  - `AutoFix-Npm.ps1` : Auto-fix for `npm not recognized` and ALN CI run.
  - `GitHub-Platform-Improvements.ps1` : Environment/bootstrap helpers.
- `.github/workflows/` : GitHub Actions workflow skeletons and CI logic.

## Node Tooling
Minimal package.json includes scripts:
```jsonc
"scripts": {
  "aln:projection": "node scripts/aln-to-json-projection.cjs",
  "aln:validate": "node scripts/aln-ajv-mesh-sweep.cjs",
  "aln:severity-gate": "node scripts/aln-severity-gate.cjs",
  "aln:metatest": "node scripts/aln-copilot-metatest.cjs"
}
```

Install and run locally:
```powershell
cd "C:\Users\Hunter\Repos\Github-Solutions"
npm install
npm run aln:projection
npm run aln:validate
npm run aln:severity-gate
npm run aln:metatest
```

## PowerShell Scripts
### AutoFix-Npm.ps1
Ensures Node + npm present, then runs projection / validation / severity gate.
```powershell
pwsh -File scripts/AutoFix-Npm.ps1 -RepoPath "C:\Users\Hunter\Repos\Github-Solutions"
```
Use `-SkipInstall` to skip `npm install` if already done.

### GitHub-Platform-Improvements.ps1
Bootstraps git, gh, node, dotnet and loads helper functions.
```powershell
pwsh -File scripts/GitHub-Platform-Improvements.ps1 -RepoPath "$PWD" -UserName "Your Name" -UserEmail "you@example.com"
```
After running, available functions: `Invoke-GitCommitPush`, `Invoke-GitHubAuth`, `Show-GitHubRepoInfo`.

### Inspect-Wasm.ps1
Inspect a WASM binary:
```powershell
pwsh -File scripts/Inspect-Wasm.ps1 -WasmPath build/module.wasm
```
Requires `wasm-objdump` on PATH (Binaryen/WABT tooling).

## CI Workflows Overview
- `aln-ci-core.yml` : Core ALN validation (non-Python environment enforcement).
- `aln-device-twin-ci.yml` : Firmware + virtual hardware simulation matrix.
- `aln-vm-bootstrap-validate.yml` : VM bootstrap validation.
- `aln-copilot-governance.yml` : Repo policy + metaprompt tests.
- `aln-telemetry-export.yml` : Telemetry aggregation.
- `aln-firmware-update-lane.yml` : Staged firmware rollout rings.

## Governance & Safety
- Python runtimes are actively blocked in CI workflows.
- Severity gate: critical violations = fail, high violations soft cap = 10.
- Copilot metaprompt tests ensure required governance commands remain present.

## Typical Local Workflow
```powershell
# Bootstrap environment
pwsh -File scripts/GitHub-Platform-Improvements.ps1 -RepoPath "$PWD" -UserName "Dev" -UserEmail "dev@example.com"

# Install dependencies and validate ALN
npm install
npm run aln:projection
npm run aln:validate
npm run aln:severity-gate

# Run metaprompt tests
npm run aln:metatest
```

## Troubleshooting
- If `npm` not recognized after winget install, open a NEW PowerShell window.
- If Ajv errors appear, inspect `reports/aln-constraint-report.json`.
- Use `scripts/Inspect-Wasm.ps1` to verify WASM artifacts when adding simulation steps.

## Next Enhancements (Optional)
- Add artifact upload to firmware/twin workflow for WASM logs.
- Implement full ALN parser replacing regex stubs.
- Integrate signing/verification for firmware images.

---
Generated on 2025-11-27.