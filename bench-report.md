# lpv vs lpv-rs timing

Reference: `php $LPV_PACKAGES/lean-package-validator.phar`  
Candidate: `<repo>/target/release/lpv-rs`  
Generated: 2026-10-23 23:17:41 MST; 15 timed runs after 2 warm-ups per cell; wall-clock medians of whole process runs.

| Mode | Package | lpv median (ms) | lpv-rs median (ms) | Speedup | Exit codes lpv/lpv-rs |
|------|---------|----------------:|-------------------:|--------:|------|
| `validate` | conduit-dj | 33.8 | 0.65 | 51.7x | 1/1 |
| `validate` | conduit-docker | 33.2 | 0.65 | 50.8x | 1/1 |
| `validate` | conduit-env-manager | 33.3 | 0.59 | 56.0x | 1/1 |
| `validate` | conduit-interfaces | 34.6 | 0.73 | 47.1x | 1/1 |
| `validate` | conduit-spotify | 34.0 | 0.66 | 51.7x | 1/1 |
| `validate` | conduit-test-dev-component | 33.7 | 0.60 | 56.4x | 1/1 |
| `validate` | conduit-test-scaffold | 32.9 | 0.60 | 54.6x | 1/1 |
| `validate` | github-client | 33.9 | 0.74 | 45.7x | 1/1 |
| `validate` | github-zero | 33.9 | 0.66 | 51.0x | 1/1 |
| `validate` | laravel-ai-parallel-tools | 34.4 | 0.66 | 52.1x | 1/1 |
| `validate` | laravel-say-logger | 33.2 | 0.58 | 57.4x | 1/1 |
| `validate` | packagist-client | 32.8 | 0.74 | 44.4x | 1/1 |
| `validate` | spotify-client | 33.6 | 0.72 | 46.9x | 1/1 |
| `validate --validate-git-archive` | conduit-dj | 40.1 | 2.76 | 14.5x | 1/1 |
| `validate --validate-git-archive` | conduit-docker | 38.2 | 2.41 | 15.8x | 1/1 |
| `validate --validate-git-archive` | conduit-env-manager | 38.6 | 2.38 | 16.2x | 1/1 |
| `validate --validate-git-archive` | conduit-interfaces | 39.2 | 2.42 | 16.2x | 1/1 |
| `validate --validate-git-archive` | conduit-spotify | 41.1 | 2.79 | 14.8x | 1/1 |
| `validate --validate-git-archive` | conduit-test-dev-component | 38.4 | 2.26 | 17.0x | 1/1 |
| `validate --validate-git-archive` | conduit-test-scaffold | 38.6 | 2.35 | 16.4x | 1/1 |
| `validate --validate-git-archive` | github-client | 42.4 | 3.54 | 12.0x | 1/1 |
| `validate --validate-git-archive` | github-zero | 38.4 | 2.55 | 15.1x | 1/1 |
| `validate --validate-git-archive` | laravel-ai-parallel-tools | 38.8 | 2.71 | 14.3x | 1/1 |
| `validate --validate-git-archive` | laravel-say-logger | 38.6 | 2.32 | 16.6x | 1/1 |
| `validate --validate-git-archive` | packagist-client | 39.5 | 2.50 | 15.8x | 1/1 |
| `validate --validate-git-archive` | spotify-client | 41.5 | 2.89 | 14.4x | 1/1 |
| `validate` | **median of packages** | **33.7** | **0.66** | **51.3x** | |
| `validate --validate-git-archive` | **median of packages** | **38.8** | **2.50** | **15.5x** | |
