# lpv-rs parity report

Reference: `php $LPV_PACKAGES/lean-package-validator.phar`  
Candidate: `<repo>/target/release/lpv-rs`  
Generated: 2026-10-23 23:19:14 MST

Comparison is byte-exact on exit code, stdout, stderr and the resulting `.gitattributes` bytes.

**627 pass, 0 fail, 7 expected-unsupported, 634 total.**

| # | Case | Result | Notes |
|---|------|--------|-------|
| 1 | `fixture:ai-files [in] validate` | PASS |  |
| 2 | `fixture:ai-files [in] validate --diff` | PASS |  |
| 3 | `fixture:ai-files [in] validate --validate-git-archive` | PASS |  |
| 4 | `fixture:ai-files [in] AI_AGENT validate` | PASS |  |
| 5 | `fixture:ai-files [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 6 | `fixture:ai-files [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 7 | `fixture:ai-files [in] create --dry-run` | PASS |  |
| 8 | `fixture:ai-files [in] update --dry-run` | PASS |  |
| 9 | `fixture:ai-files [in] update` | PASS |  |
| 10 | `fixture:ai-files [in] create --force` | PASS |  |
| 11 | `fixture:ai-files [in] create` | PASS |  |
| 12 | `fixture:ai-files [in] AI_AGENT update` | PASS |  |
| 13 | `fixture:aligned [in] validate` | PASS |  |
| 14 | `fixture:aligned [in] validate --diff` | PASS |  |
| 15 | `fixture:aligned [in] validate --validate-git-archive` | PASS |  |
| 16 | `fixture:aligned [in] AI_AGENT validate` | PASS |  |
| 17 | `fixture:aligned [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 18 | `fixture:aligned [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 19 | `fixture:aligned [in] create --dry-run` | PASS |  |
| 20 | `fixture:aligned [in] update --dry-run` | PASS |  |
| 21 | `fixture:aligned [in] update` | PASS |  |
| 22 | `fixture:aligned [in] create --force` | PASS |  |
| 23 | `fixture:aligned [in] create` | PASS |  |
| 24 | `fixture:aligned [in] AI_AGENT update` | PASS |  |
| 25 | `fixture:archive-leaks [in] validate` | PASS |  |
| 26 | `fixture:archive-leaks [in] validate --diff` | PASS |  |
| 27 | `fixture:archive-leaks [in] validate --validate-git-archive` | PASS |  |
| 28 | `fixture:archive-leaks [in] AI_AGENT validate` | PASS |  |
| 29 | `fixture:archive-leaks [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 30 | `fixture:archive-leaks [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 31 | `fixture:archive-leaks [in] create --dry-run` | PASS |  |
| 32 | `fixture:archive-leaks [in] update --dry-run` | PASS |  |
| 33 | `fixture:archive-leaks [in] update` | PASS |  |
| 34 | `fixture:archive-leaks [in] create --force` | PASS |  |
| 35 | `fixture:archive-leaks [in] create` | PASS |  |
| 36 | `fixture:archive-leaks [in] AI_AGENT update` | PASS |  |
| 37 | `fixture:bare [in] validate` | PASS |  |
| 38 | `fixture:bare [in] validate --diff` | PASS |  |
| 39 | `fixture:bare [in] validate --validate-git-archive` | PASS |  |
| 40 | `fixture:bare [in] AI_AGENT validate` | PASS |  |
| 41 | `fixture:bare [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 42 | `fixture:bare [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 43 | `fixture:bare [in] create --dry-run` | PASS |  |
| 44 | `fixture:bare [in] update --dry-run` | PASS |  |
| 45 | `fixture:bare [in] update` | PASS |  |
| 46 | `fixture:bare [in] create --force` | PASS |  |
| 47 | `fixture:bare [in] create` | PASS |  |
| 48 | `fixture:bare [in] AI_AGENT update` | PASS |  |
| 49 | `fixture:crlf [in] validate` | PASS |  |
| 50 | `fixture:crlf [in] validate --diff` | PASS |  |
| 51 | `fixture:crlf [in] validate --validate-git-archive` | PASS |  |
| 52 | `fixture:crlf [in] AI_AGENT validate` | PASS |  |
| 53 | `fixture:crlf [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 54 | `fixture:crlf [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 55 | `fixture:crlf [in] create --dry-run` | PASS |  |
| 56 | `fixture:crlf [in] update --dry-run` | PASS |  |
| 57 | `fixture:crlf [in] update` | PASS |  |
| 58 | `fixture:crlf [in] create --force` | PASS |  |
| 59 | `fixture:crlf [in] create` | PASS |  |
| 60 | `fixture:crlf [in] AI_AGENT update` | PASS |  |
| 61 | `fixture:generated-header [in] validate` | PASS |  |
| 62 | `fixture:generated-header [in] validate --diff` | PASS |  |
| 63 | `fixture:generated-header [in] validate --validate-git-archive` | PASS |  |
| 64 | `fixture:generated-header [in] AI_AGENT validate` | PASS |  |
| 65 | `fixture:generated-header [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 66 | `fixture:generated-header [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 67 | `fixture:generated-header [in] create --dry-run` | PASS |  |
| 68 | `fixture:generated-header [in] update --dry-run` | PASS |  |
| 69 | `fixture:generated-header [in] update` | PASS |  |
| 70 | `fixture:generated-header [in] create --force` | PASS |  |
| 71 | `fixture:generated-header [in] create` | PASS |  |
| 72 | `fixture:generated-header [in] AI_AGENT update` | PASS |  |
| 73 | `fixture:gitignore [in] validate` | PASS |  |
| 74 | `fixture:gitignore [in] validate --diff` | PASS |  |
| 75 | `fixture:gitignore [in] validate --validate-git-archive` | PASS |  |
| 76 | `fixture:gitignore [in] AI_AGENT validate` | PASS |  |
| 77 | `fixture:gitignore [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 78 | `fixture:gitignore [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 79 | `fixture:gitignore [in] create --dry-run` | PASS |  |
| 80 | `fixture:gitignore [in] update --dry-run` | PASS |  |
| 81 | `fixture:gitignore [in] update` | PASS |  |
| 82 | `fixture:gitignore [in] create --force` | PASS |  |
| 83 | `fixture:gitignore [in] create` | PASS |  |
| 84 | `fixture:gitignore [in] AI_AGENT update` | PASS |  |
| 85 | `fixture:keep-license [in] validate` | PASS |  |
| 86 | `fixture:keep-license [in] validate --diff` | PASS |  |
| 87 | `fixture:keep-license [in] validate --validate-git-archive` | PASS |  |
| 88 | `fixture:keep-license [in] AI_AGENT validate` | PASS |  |
| 89 | `fixture:keep-license [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 90 | `fixture:keep-license [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 91 | `fixture:keep-license [in] create --dry-run` | PASS |  |
| 92 | `fixture:keep-license [in] update --dry-run` | PASS |  |
| 93 | `fixture:keep-license [in] update` | PASS |  |
| 94 | `fixture:keep-license [in] create --force` | PASS |  |
| 95 | `fixture:keep-license [in] create` | PASS |  |
| 96 | `fixture:keep-license [in] AI_AGENT update` | PASS |  |
| 97 | `fixture:keep-readme [in] validate` | PASS |  |
| 98 | `fixture:keep-readme [in] validate --diff` | PASS |  |
| 99 | `fixture:keep-readme [in] validate --validate-git-archive` | PASS |  |
| 100 | `fixture:keep-readme [in] AI_AGENT validate` | PASS |  |
| 101 | `fixture:keep-readme [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 102 | `fixture:keep-readme [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 103 | `fixture:keep-readme [in] create --dry-run` | PASS |  |
| 104 | `fixture:keep-readme [in] update --dry-run` | PASS |  |
| 105 | `fixture:keep-readme [in] update` | PASS |  |
| 106 | `fixture:keep-readme [in] create --force` | PASS |  |
| 107 | `fixture:keep-readme [in] create` | PASS |  |
| 108 | `fixture:keep-readme [in] AI_AGENT update` | PASS |  |
| 109 | `fixture:lpv-file [in] validate` | PASS |  |
| 110 | `fixture:lpv-file [in] validate --diff` | PASS |  |
| 111 | `fixture:lpv-file [in] validate --validate-git-archive` | PASS |  |
| 112 | `fixture:lpv-file [in] AI_AGENT validate` | PASS |  |
| 113 | `fixture:lpv-file [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 114 | `fixture:lpv-file [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 115 | `fixture:lpv-file [in] create --dry-run` | PASS |  |
| 116 | `fixture:lpv-file [in] update --dry-run` | PASS |  |
| 117 | `fixture:lpv-file [in] update` | PASS |  |
| 118 | `fixture:lpv-file [in] create --force` | PASS |  |
| 119 | `fixture:lpv-file [in] create` | PASS |  |
| 120 | `fixture:lpv-file [in] AI_AGENT update` | PASS |  |
| 121 | `fixture:missing-ignores [in] validate` | PASS |  |
| 122 | `fixture:missing-ignores [in] validate --diff` | PASS |  |
| 123 | `fixture:missing-ignores [in] validate --validate-git-archive` | PASS |  |
| 124 | `fixture:missing-ignores [in] AI_AGENT validate` | PASS |  |
| 125 | `fixture:missing-ignores [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 126 | `fixture:missing-ignores [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 127 | `fixture:missing-ignores [in] create --dry-run` | PASS |  |
| 128 | `fixture:missing-ignores [in] update --dry-run` | PASS |  |
| 129 | `fixture:missing-ignores [in] update` | PASS |  |
| 130 | `fixture:missing-ignores [in] create --force` | PASS |  |
| 131 | `fixture:missing-ignores [in] create` | PASS |  |
| 132 | `fixture:missing-ignores [in] AI_AGENT update` | PASS |  |
| 133 | `fixture:negated [in] validate` | PASS |  |
| 134 | `fixture:negated [in] validate --diff` | PASS |  |
| 135 | `fixture:negated [in] validate --validate-git-archive` | PASS |  |
| 136 | `fixture:negated [in] AI_AGENT validate` | PASS |  |
| 137 | `fixture:negated [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 138 | `fixture:negated [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 139 | `fixture:negated [in] create --dry-run` | PASS |  |
| 140 | `fixture:negated [in] update --dry-run` | PASS |  |
| 141 | `fixture:negated [in] update` | PASS |  |
| 142 | `fixture:negated [in] create --force` | PASS |  |
| 143 | `fixture:negated [in] create` | PASS |  |
| 144 | `fixture:negated [in] AI_AGENT update` | PASS |  |
| 145 | `fixture:negated-bin [in] validate` | PASS |  |
| 146 | `fixture:negated-bin [in] validate --diff` | PASS |  |
| 147 | `fixture:negated-bin [in] validate --validate-git-archive` | PASS |  |
| 148 | `fixture:negated-bin [in] AI_AGENT validate` | PASS |  |
| 149 | `fixture:negated-bin [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 150 | `fixture:negated-bin [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 151 | `fixture:negated-bin [in] create --dry-run` | PASS |  |
| 152 | `fixture:negated-bin [in] update --dry-run` | PASS |  |
| 153 | `fixture:negated-bin [in] update` | PASS |  |
| 154 | `fixture:negated-bin [in] create --force` | PASS |  |
| 155 | `fixture:negated-bin [in] create` | PASS |  |
| 156 | `fixture:negated-bin [in] AI_AGENT update` | PASS |  |
| 157 | `fixture:negated-invalid [in] validate` | PASS |  |
| 158 | `fixture:negated-invalid [in] validate --diff` | PASS |  |
| 159 | `fixture:negated-invalid [in] validate --validate-git-archive` | PASS |  |
| 160 | `fixture:negated-invalid [in] AI_AGENT validate` | PASS |  |
| 161 | `fixture:negated-invalid [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 162 | `fixture:negated-invalid [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 163 | `fixture:negated-invalid [in] create --dry-run` | PASS |  |
| 164 | `fixture:negated-invalid [in] update --dry-run` | PASS |  |
| 165 | `fixture:negated-invalid [in] update` | PASS |  |
| 166 | `fixture:negated-invalid [in] create --force` | PASS |  |
| 167 | `fixture:negated-invalid [in] create` | PASS |  |
| 168 | `fixture:negated-invalid [in] AI_AGENT update` | PASS |  |
| 169 | `fixture:no-gitattributes [in] validate` | PASS |  |
| 170 | `fixture:no-gitattributes [in] validate --diff` | PASS |  |
| 171 | `fixture:no-gitattributes [in] validate --validate-git-archive` | PASS |  |
| 172 | `fixture:no-gitattributes [in] AI_AGENT validate` | PASS |  |
| 173 | `fixture:no-gitattributes [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 174 | `fixture:no-gitattributes [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 175 | `fixture:no-gitattributes [in] create --dry-run` | PASS |  |
| 176 | `fixture:no-gitattributes [in] update --dry-run` | PASS |  |
| 177 | `fixture:no-gitattributes [in] update` | PASS |  |
| 178 | `fixture:no-gitattributes [in] create --force` | PASS |  |
| 179 | `fixture:no-gitattributes [in] create` | PASS |  |
| 180 | `fixture:no-gitattributes [in] AI_AGENT update` | PASS |  |
| 181 | `fixture:no-license-file [in] validate` | PASS |  |
| 182 | `fixture:no-license-file [in] validate --diff` | PASS |  |
| 183 | `fixture:no-license-file [in] validate --validate-git-archive` | PASS |  |
| 184 | `fixture:no-license-file [in] AI_AGENT validate` | PASS |  |
| 185 | `fixture:no-license-file [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 186 | `fixture:no-license-file [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 187 | `fixture:no-license-file [in] create --dry-run` | PASS |  |
| 188 | `fixture:no-license-file [in] update --dry-run` | PASS |  |
| 189 | `fixture:no-license-file [in] update` | PASS |  |
| 190 | `fixture:no-license-file [in] create --force` | PASS |  |
| 191 | `fixture:no-license-file [in] create` | PASS |  |
| 192 | `fixture:no-license-file [in] AI_AGENT update` | PASS |  |
| 193 | `fixture:preserve-nested [in] validate` | PASS |  |
| 194 | `fixture:preserve-nested [in] validate --diff` | PASS |  |
| 195 | `fixture:preserve-nested [in] validate --validate-git-archive` | PASS |  |
| 196 | `fixture:preserve-nested [in] AI_AGENT validate` | PASS |  |
| 197 | `fixture:preserve-nested [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 198 | `fixture:preserve-nested [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 199 | `fixture:preserve-nested [in] create --dry-run` | PASS |  |
| 200 | `fixture:preserve-nested [in] update --dry-run` | PASS |  |
| 201 | `fixture:preserve-nested [in] update` | PASS |  |
| 202 | `fixture:preserve-nested [in] create --force` | PASS |  |
| 203 | `fixture:preserve-nested [in] create` | PASS |  |
| 204 | `fixture:preserve-nested [in] AI_AGENT update` | PASS |  |
| 205 | `fixture:python-preset [in] validate` | PASS |  |
| 206 | `fixture:python-preset [in] validate --diff` | PASS |  |
| 207 | `fixture:python-preset [in] validate --validate-git-archive` | PASS |  |
| 208 | `fixture:python-preset [in] AI_AGENT validate` | PASS |  |
| 209 | `fixture:python-preset [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 210 | `fixture:python-preset [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 211 | `fixture:python-preset [in] create --dry-run` | PASS |  |
| 212 | `fixture:python-preset [in] update --dry-run` | PASS |  |
| 213 | `fixture:python-preset [in] update` | PASS |  |
| 214 | `fixture:python-preset [in] create --force` | PASS |  |
| 215 | `fixture:python-preset [in] create` | PASS |  |
| 216 | `fixture:python-preset [in] AI_AGENT update` | PASS |  |
| 217 | `fixture:stale-ignores [in] validate` | PASS |  |
| 218 | `fixture:stale-ignores [in] validate --diff` | PASS |  |
| 219 | `fixture:stale-ignores [in] validate --validate-git-archive` | PASS |  |
| 220 | `fixture:stale-ignores [in] AI_AGENT validate` | PASS |  |
| 221 | `fixture:stale-ignores [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 222 | `fixture:stale-ignores [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 223 | `fixture:stale-ignores [in] create --dry-run` | PASS |  |
| 224 | `fixture:stale-ignores [in] update --dry-run` | PASS |  |
| 225 | `fixture:stale-ignores [in] update` | PASS |  |
| 226 | `fixture:stale-ignores [in] create --force` | PASS |  |
| 227 | `fixture:stale-ignores [in] create` | PASS |  |
| 228 | `fixture:stale-ignores [in] AI_AGENT update` | PASS |  |
| 229 | `fixture:unsorted [in] validate` | PASS |  |
| 230 | `fixture:unsorted [in] validate --diff` | PASS |  |
| 231 | `fixture:unsorted [in] validate --validate-git-archive` | PASS |  |
| 232 | `fixture:unsorted [in] AI_AGENT validate` | PASS |  |
| 233 | `fixture:unsorted [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 234 | `fixture:unsorted [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 235 | `fixture:unsorted [in] create --dry-run` | PASS |  |
| 236 | `fixture:unsorted [in] update --dry-run` | PASS |  |
| 237 | `fixture:unsorted [in] update` | PASS |  |
| 238 | `fixture:unsorted [in] create --force` | PASS |  |
| 239 | `fixture:unsorted [in] create` | PASS |  |
| 240 | `fixture:unsorted [in] AI_AGENT update` | PASS |  |
| 241 | `fixture:valid [in] validate` | PASS |  |
| 242 | `fixture:valid [in] validate --diff` | PASS |  |
| 243 | `fixture:valid [in] validate --validate-git-archive` | PASS |  |
| 244 | `fixture:valid [in] AI_AGENT validate` | PASS |  |
| 245 | `fixture:valid [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 246 | `fixture:valid [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 247 | `fixture:valid [in] create --dry-run` | PASS |  |
| 248 | `fixture:valid [in] update --dry-run` | PASS |  |
| 249 | `fixture:valid [in] update` | PASS |  |
| 250 | `fixture:valid [in] create --force` | PASS |  |
| 251 | `fixture:valid [in] create` | PASS |  |
| 252 | `fixture:valid [in] AI_AGENT update` | PASS |  |
| 253 | `fixture:warnings [in] validate` | PASS |  |
| 254 | `fixture:warnings [in] validate --diff` | PASS |  |
| 255 | `fixture:warnings [in] validate --validate-git-archive` | PASS |  |
| 256 | `fixture:warnings [in] AI_AGENT validate` | PASS |  |
| 257 | `fixture:warnings [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 258 | `fixture:warnings [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 259 | `fixture:warnings [in] create --dry-run` | PASS |  |
| 260 | `fixture:warnings [in] update --dry-run` | PASS |  |
| 261 | `fixture:warnings [in] update` | PASS |  |
| 262 | `fixture:warnings [in] create --force` | PASS |  |
| 263 | `fixture:warnings [in] create` | PASS |  |
| 264 | `fixture:warnings [in] AI_AGENT update` | PASS |  |
| 265 | `fixture:keep-readme [in] validate --keep-readme` | PASS |  |
| 266 | `fixture:keep-readme [in] update --keep-readme` | PASS |  |
| 267 | `fixture:keep-readme [in] validate --keep-readme --validate-git-archive` | PASS |  |
| 268 | `fixture:keep-license [in] validate --keep-license` | PASS |  |
| 269 | `fixture:keep-license [in] validate --keep-license --validate-git-archive` | PASS |  |
| 270 | `fixture:keep-license [in] create --force --keep-license --dry-run` | PASS |  |
| 271 | `fixture:no-license-file [in] validate --keep-license --validate-git-archive` | PASS |  |
| 272 | `fixture:no-license-file [in] validate --keep-license` | PASS |  |
| 273 | `fixture:aligned [in] validate --enforce-alignment` | PASS |  |
| 274 | `fixture:aligned [in] update -a` | PASS |  |
| 275 | `fixture:aligned [in] create --dry-run -a` | PASS |  |
| 276 | `fixture:aligned [in] update --align-export-ignores --sort-from-directories-to-files` | PASS |  |
| 277 | `fixture:unsorted [in] validate --enforce-strict-order` | PASS |  |
| 278 | `fixture:unsorted [in] validate -s` | PASS |  |
| 279 | `fixture:unsorted [in] update -s` | PASS |  |
| 280 | `fixture:unsorted [in] validate --enforce-strict-order --stdin-input <valid.txt` | PASS |  |
| 281 | `fixture:python-preset [in] validate --preset python` | PASS |  |
| 282 | `fixture:python-preset [in] validate --preset=Python --diff` | PASS |  |
| 283 | `fixture:python-preset [in] validate --preset javascript` | PASS |  |
| 284 | `fixture:python-preset [in] validate --preset foo` | PASS |  |
| 285 | `fixture:python-preset [in] create --dry-run --preset rust` | PASS |  |
| 286 | `fixture:python-preset [in] update --preset=go --dry-run` | PASS |  |
| 287 | `fixture:python-preset [in] validate --preset` | PASS |  |
| 288 | `fixture:lpv-file [in] validate -v` | PASS |  |
| 289 | `fixture:lpv-file [in] validate --glob-pattern-file=missing.lpv` | PASS |  |
| 290 | `fixture:lpv-file [in] validate --glob-pattern-file` | PASS |  |
| 291 | `fixture:lpv-file [in] update --dry-run -v` | PASS |  |
| 292 | `fixture:valid [in] validate -v` | PASS |  |
| 293 | `fixture:valid [in] validate -vvv` | UNSUPPORTED | stderr differs |
| 294 | `fixture:valid [in] validate --glob-pattern {.*,*.md}` | PASS |  |
| 295 | `fixture:valid [in] validate --glob-pattern {x}` | PASS |  |
| 296 | `fixture:valid [in] validate --glob-pattern=` | PASS |  |
| 297 | `fixture:valid [in] validate --keep-glob-pattern` | PASS |  |
| 298 | `fixture:valid [in] validate --stdin-input <valid.txt` | PASS |  |
| 299 | `fixture:valid [in] validate --stdin-input <invalid.txt` | PASS |  |
| 300 | `fixture:valid [in] validate --stdin-input <garbage.txt` | PASS |  |
| 301 | `fixture:valid [in] validate /nonexistent/dir` | PASS |  |
| 302 | `fixture:valid [in] validate a b` | PASS |  |
| 303 | `fixture:valid [in] validate --bogus` | PASS |  |
| 304 | `fixture:valid [in] validate --glob-pattern` | PASS |  |
| 305 | `fixture:valid [in] create --keep-glob-pattern {README*,tests*} --dry-run --force` | PASS |  |
| 306 | `fixture:valid [in] update --keep-glob-pattern {x}` | PASS |  |
| 307 | `fixture:valid [in] update --group` | PASS |  |
| 308 | `fixture:valid [in] create --flavour=x` | PASS |  |
| 309 | `fixture:valid [in] create --flavour negated --dry-run` | PASS |  |
| 310 | `fixture:valid [in] create --flavour negated --force -a` | PASS |  |
| 311 | `fixture:valid [in] create --flavour negated --force --keep-readme --keep-glob-pattern {tests*}` | PASS |  |
| 312 | `fixture:valid [in] validate --report-impact` | UNSUPPORTED | exit code 0 != 1; stdout differs; stderr differs |
| 313 | `fixture:valid [in] frobnicate` | PASS |  |
| 314 | `fixture:valid [in] update --keep-readme=yes` | PASS |  |
| 315 | `fixture:valid [in] validate -q` | PASS |  |
| 316 | `fixture:valid [in] validate --no-ansi` | PASS |  |
| 317 | `fixture:valid [in] val` | PASS |  |
| 318 | `fixture:valid [in] validate --stdin-input --keep-readme <invalid.txt` | PASS |  |
| 319 | `fixture:valid [in] VALIDATE` | PASS |  |
| 320 | `fixture:valid [in] re` | PASS |  |
| 321 | `fixture:valid [in] c` | PASS |  |
| 322 | `fixture:valid [in] l` | PASS |  |
| 323 | `fixture:valid [in] x:y` | PASS |  |
| 324 | `fixture:valid [in] valdate` | PASS |  |
| 325 | `fixture:valid [in] valdate -n` | PASS |  |
| 326 | `fixture:valid [in] lst -q` | PASS |  |
| 327 | `fixture:valid [in] creat --dry-run` | PASS |  |
| 328 | `fixture:valid [in] valdate <yes.txt` | PASS |  |
| 329 | `fixture:valid [in] lst <yes.txt` | UNSUPPORTED | exit code 0 != 1; stdout differs; stderr differs |
| 330 | `fixture:crlf [in] update --group` | PASS |  |
| 331 | `fixture:crlf [in] validate --enforce-alignment --diff` | PASS |  |
| 332 | `fixture:preserve-nested [in] update --group` | PASS |  |
| 333 | `fixture:preserve-nested [in] validate --diff -s` | PASS |  |
| 334 | `fixture:preserve-nested [in] update -s` | PASS |  |
| 335 | `fixture:negated [in] validate --report-stale-export-ignores` | PASS |  |
| 336 | `fixture:negated [in] validate --stdin-input <valid.txt` | PASS |  |
| 337 | `fixture:negated [in] update -a` | PASS |  |
| 338 | `fixture:negated [in] create --force -a` | PASS |  |
| 339 | `fixture:negated [in] update --group` | PASS |  |
| 340 | `fixture:negated [in] update --sort-from-directories-to-files` | PASS |  |
| 341 | `fixture:negated-bin [in] update -a` | PASS |  |
| 342 | `fixture:negated-bin [in] create --force --keep-readme` | PASS |  |
| 343 | `fixture:negated-bin [in] create --force --keep-license` | PASS |  |
| 344 | `fixture:warnings [in] validate -v` | PASS |  |
| 345 | `fixture:stale-ignores [in] validate --report-stale-export-ignores` | PASS |  |
| 346 | `fixture:stale-ignores [in] update --group` | PASS |  |
| 347 | `fixture:generated-header [in] update -a` | PASS |  |
| 348 | `fixture:gitignore [in] validate -v` | PASS |  |
| 349 | `fixture:gitignore [in] create --force --dry-run` | PASS |  |
| 350 | `fixture:ai-files [in] validate -s --diff` | PASS |  |
| 351 | `fixture:ai-files [in] update -a -s` | PASS |  |
| 352 | `fixture:valid [parent] validate` | PASS |  |
| 353 | `fixture:valid [parent] validate --report-stale-export-ignores --diff` | PASS |  |
| 354 | `fixture:valid [parent] validate --validate-git-archive` | PASS |  |
| 355 | `fixture:valid [parent] create --dry-run` | PASS |  |
| 356 | `fixture:valid [parent] update` | PASS |  |
| 357 | `fixture:valid [parent] update --keep-license` | PASS |  |
| 358 | `fixture:valid [parent] create --force --keep-license` | PASS |  |
| 359 | `fixture:stale-ignores [parent] validate` | PASS |  |
| 360 | `fixture:stale-ignores [parent] validate --report-stale-export-ignores --diff` | PASS |  |
| 361 | `fixture:stale-ignores [parent] validate --validate-git-archive` | PASS |  |
| 362 | `fixture:stale-ignores [parent] create --dry-run` | PASS |  |
| 363 | `fixture:stale-ignores [parent] update` | PASS |  |
| 364 | `fixture:stale-ignores [parent] update --keep-license` | PASS |  |
| 365 | `fixture:stale-ignores [parent] create --force --keep-license` | PASS |  |
| 366 | `fixture:missing-ignores [parent] validate` | PASS |  |
| 367 | `fixture:missing-ignores [parent] validate --report-stale-export-ignores --diff` | PASS |  |
| 368 | `fixture:missing-ignores [parent] validate --validate-git-archive` | PASS |  |
| 369 | `fixture:missing-ignores [parent] create --dry-run` | PASS |  |
| 370 | `fixture:missing-ignores [parent] update` | PASS |  |
| 371 | `fixture:missing-ignores [parent] update --keep-license` | PASS |  |
| 372 | `fixture:missing-ignores [parent] create --force --keep-license` | PASS |  |
| 373 | `fixture:no-gitattributes [parent] validate` | PASS |  |
| 374 | `fixture:no-gitattributes [parent] validate --report-stale-export-ignores --diff` | PASS |  |
| 375 | `fixture:no-gitattributes [parent] validate --validate-git-archive` | PASS |  |
| 376 | `fixture:no-gitattributes [parent] create --dry-run` | PASS |  |
| 377 | `fixture:no-gitattributes [parent] update` | PASS |  |
| 378 | `fixture:no-gitattributes [parent] update --keep-license` | PASS |  |
| 379 | `fixture:no-gitattributes [parent] create --force --keep-license` | PASS |  |
| 380 | `fixture:lpv-file [parent] validate` | PASS |  |
| 381 | `fixture:lpv-file [parent] validate --report-stale-export-ignores --diff` | PASS |  |
| 382 | `fixture:lpv-file [parent] validate --validate-git-archive` | PASS |  |
| 383 | `fixture:lpv-file [parent] create --dry-run` | PASS |  |
| 384 | `fixture:lpv-file [parent] update` | PASS |  |
| 385 | `fixture:lpv-file [parent] update --keep-license` | PASS |  |
| 386 | `fixture:lpv-file [parent] create --force --keep-license` | PASS |  |
| 387 | `fixture:negated [parent] validate` | PASS |  |
| 388 | `fixture:negated [parent] validate --report-stale-export-ignores --diff` | PASS |  |
| 389 | `fixture:negated [parent] validate --validate-git-archive` | PASS |  |
| 390 | `fixture:negated [parent] create --dry-run` | PASS |  |
| 391 | `fixture:negated [parent] update` | PASS |  |
| 392 | `fixture:negated [parent] update --keep-license` | UNSUPPORTED | stderr differs |
| 393 | `fixture:negated [parent] create --force --keep-license` | UNSUPPORTED | stderr differs |
| 394 | `fixture:negated-bin [parent] validate` | PASS |  |
| 395 | `fixture:negated-bin [parent] validate --report-stale-export-ignores --diff` | PASS |  |
| 396 | `fixture:negated-bin [parent] validate --validate-git-archive` | PASS |  |
| 397 | `fixture:negated-bin [parent] create --dry-run` | PASS |  |
| 398 | `fixture:negated-bin [parent] update` | PASS |  |
| 399 | `fixture:negated-bin [parent] update --keep-license` | UNSUPPORTED | stderr differs |
| 400 | `fixture:negated-bin [parent] create --force --keep-license` | UNSUPPORTED | stderr differs |
| 401 | `pkg:conduit-dj [in] validate` | PASS |  |
| 402 | `pkg:conduit-dj [in] validate --diff` | PASS |  |
| 403 | `pkg:conduit-dj [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 404 | `pkg:conduit-dj [in] validate --enforce-alignment` | PASS |  |
| 405 | `pkg:conduit-dj [in] validate -s` | PASS |  |
| 406 | `pkg:conduit-dj [in] validate --enforce-strict-order` | PASS |  |
| 407 | `pkg:conduit-dj [in] validate --keep-readme --keep-license` | PASS |  |
| 408 | `pkg:conduit-dj [in] validate --validate-git-archive` | PASS |  |
| 409 | `pkg:conduit-dj [in] AI_AGENT validate` | PASS |  |
| 410 | `pkg:conduit-dj [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 411 | `pkg:conduit-dj [in] validate -v` | PASS |  |
| 412 | `pkg:conduit-dj [in] create --dry-run` | PASS |  |
| 413 | `pkg:conduit-dj [in] update --dry-run` | PASS |  |
| 414 | `pkg:conduit-dj [in] update` | PASS |  |
| 415 | `pkg:conduit-dj [in] update -a` | PASS |  |
| 416 | `pkg:conduit-dj [in] update --group` | PASS |  |
| 417 | `pkg:conduit-dj [in] create --force` | PASS |  |
| 418 | `pkg:conduit-dj [in] AI_AGENT update` | PASS |  |
| 419 | `pkg:conduit-docker [in] validate` | PASS |  |
| 420 | `pkg:conduit-docker [in] validate --diff` | PASS |  |
| 421 | `pkg:conduit-docker [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 422 | `pkg:conduit-docker [in] validate --enforce-alignment` | PASS |  |
| 423 | `pkg:conduit-docker [in] validate -s` | PASS |  |
| 424 | `pkg:conduit-docker [in] validate --enforce-strict-order` | PASS |  |
| 425 | `pkg:conduit-docker [in] validate --keep-readme --keep-license` | PASS |  |
| 426 | `pkg:conduit-docker [in] validate --validate-git-archive` | PASS |  |
| 427 | `pkg:conduit-docker [in] AI_AGENT validate` | PASS |  |
| 428 | `pkg:conduit-docker [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 429 | `pkg:conduit-docker [in] validate -v` | PASS |  |
| 430 | `pkg:conduit-docker [in] create --dry-run` | PASS |  |
| 431 | `pkg:conduit-docker [in] update --dry-run` | PASS |  |
| 432 | `pkg:conduit-docker [in] update` | PASS |  |
| 433 | `pkg:conduit-docker [in] update -a` | PASS |  |
| 434 | `pkg:conduit-docker [in] update --group` | PASS |  |
| 435 | `pkg:conduit-docker [in] create --force` | PASS |  |
| 436 | `pkg:conduit-docker [in] AI_AGENT update` | PASS |  |
| 437 | `pkg:conduit-env-manager [in] validate` | PASS |  |
| 438 | `pkg:conduit-env-manager [in] validate --diff` | PASS |  |
| 439 | `pkg:conduit-env-manager [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 440 | `pkg:conduit-env-manager [in] validate --enforce-alignment` | PASS |  |
| 441 | `pkg:conduit-env-manager [in] validate -s` | PASS |  |
| 442 | `pkg:conduit-env-manager [in] validate --enforce-strict-order` | PASS |  |
| 443 | `pkg:conduit-env-manager [in] validate --keep-readme --keep-license` | PASS |  |
| 444 | `pkg:conduit-env-manager [in] validate --validate-git-archive` | PASS |  |
| 445 | `pkg:conduit-env-manager [in] AI_AGENT validate` | PASS |  |
| 446 | `pkg:conduit-env-manager [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 447 | `pkg:conduit-env-manager [in] validate -v` | PASS |  |
| 448 | `pkg:conduit-env-manager [in] create --dry-run` | PASS |  |
| 449 | `pkg:conduit-env-manager [in] update --dry-run` | PASS |  |
| 450 | `pkg:conduit-env-manager [in] update` | PASS |  |
| 451 | `pkg:conduit-env-manager [in] update -a` | PASS |  |
| 452 | `pkg:conduit-env-manager [in] update --group` | PASS |  |
| 453 | `pkg:conduit-env-manager [in] create --force` | PASS |  |
| 454 | `pkg:conduit-env-manager [in] AI_AGENT update` | PASS |  |
| 455 | `pkg:conduit-interfaces [in] validate` | PASS |  |
| 456 | `pkg:conduit-interfaces [in] validate --diff` | PASS |  |
| 457 | `pkg:conduit-interfaces [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 458 | `pkg:conduit-interfaces [in] validate --enforce-alignment` | PASS |  |
| 459 | `pkg:conduit-interfaces [in] validate -s` | PASS |  |
| 460 | `pkg:conduit-interfaces [in] validate --enforce-strict-order` | PASS |  |
| 461 | `pkg:conduit-interfaces [in] validate --keep-readme --keep-license` | PASS |  |
| 462 | `pkg:conduit-interfaces [in] validate --validate-git-archive` | PASS |  |
| 463 | `pkg:conduit-interfaces [in] AI_AGENT validate` | PASS |  |
| 464 | `pkg:conduit-interfaces [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 465 | `pkg:conduit-interfaces [in] validate -v` | PASS |  |
| 466 | `pkg:conduit-interfaces [in] create --dry-run` | PASS |  |
| 467 | `pkg:conduit-interfaces [in] update --dry-run` | PASS |  |
| 468 | `pkg:conduit-interfaces [in] update` | PASS |  |
| 469 | `pkg:conduit-interfaces [in] update -a` | PASS |  |
| 470 | `pkg:conduit-interfaces [in] update --group` | PASS |  |
| 471 | `pkg:conduit-interfaces [in] create --force` | PASS |  |
| 472 | `pkg:conduit-interfaces [in] AI_AGENT update` | PASS |  |
| 473 | `pkg:conduit-spotify [in] validate` | PASS |  |
| 474 | `pkg:conduit-spotify [in] validate --diff` | PASS |  |
| 475 | `pkg:conduit-spotify [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 476 | `pkg:conduit-spotify [in] validate --enforce-alignment` | PASS |  |
| 477 | `pkg:conduit-spotify [in] validate -s` | PASS |  |
| 478 | `pkg:conduit-spotify [in] validate --enforce-strict-order` | PASS |  |
| 479 | `pkg:conduit-spotify [in] validate --keep-readme --keep-license` | PASS |  |
| 480 | `pkg:conduit-spotify [in] validate --validate-git-archive` | PASS |  |
| 481 | `pkg:conduit-spotify [in] AI_AGENT validate` | PASS |  |
| 482 | `pkg:conduit-spotify [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 483 | `pkg:conduit-spotify [in] validate -v` | PASS |  |
| 484 | `pkg:conduit-spotify [in] create --dry-run` | PASS |  |
| 485 | `pkg:conduit-spotify [in] update --dry-run` | PASS |  |
| 486 | `pkg:conduit-spotify [in] update` | PASS |  |
| 487 | `pkg:conduit-spotify [in] update -a` | PASS |  |
| 488 | `pkg:conduit-spotify [in] update --group` | PASS |  |
| 489 | `pkg:conduit-spotify [in] create --force` | PASS |  |
| 490 | `pkg:conduit-spotify [in] AI_AGENT update` | PASS |  |
| 491 | `pkg:conduit-test-dev-component [in] validate` | PASS |  |
| 492 | `pkg:conduit-test-dev-component [in] validate --diff` | PASS |  |
| 493 | `pkg:conduit-test-dev-component [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 494 | `pkg:conduit-test-dev-component [in] validate --enforce-alignment` | PASS |  |
| 495 | `pkg:conduit-test-dev-component [in] validate -s` | PASS |  |
| 496 | `pkg:conduit-test-dev-component [in] validate --enforce-strict-order` | PASS |  |
| 497 | `pkg:conduit-test-dev-component [in] validate --keep-readme --keep-license` | PASS |  |
| 498 | `pkg:conduit-test-dev-component [in] validate --validate-git-archive` | PASS |  |
| 499 | `pkg:conduit-test-dev-component [in] AI_AGENT validate` | PASS |  |
| 500 | `pkg:conduit-test-dev-component [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 501 | `pkg:conduit-test-dev-component [in] validate -v` | PASS |  |
| 502 | `pkg:conduit-test-dev-component [in] create --dry-run` | PASS |  |
| 503 | `pkg:conduit-test-dev-component [in] update --dry-run` | PASS |  |
| 504 | `pkg:conduit-test-dev-component [in] update` | PASS |  |
| 505 | `pkg:conduit-test-dev-component [in] update -a` | PASS |  |
| 506 | `pkg:conduit-test-dev-component [in] update --group` | PASS |  |
| 507 | `pkg:conduit-test-dev-component [in] create --force` | PASS |  |
| 508 | `pkg:conduit-test-dev-component [in] AI_AGENT update` | PASS |  |
| 509 | `pkg:conduit-test-scaffold [in] validate` | PASS |  |
| 510 | `pkg:conduit-test-scaffold [in] validate --diff` | PASS |  |
| 511 | `pkg:conduit-test-scaffold [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 512 | `pkg:conduit-test-scaffold [in] validate --enforce-alignment` | PASS |  |
| 513 | `pkg:conduit-test-scaffold [in] validate -s` | PASS |  |
| 514 | `pkg:conduit-test-scaffold [in] validate --enforce-strict-order` | PASS |  |
| 515 | `pkg:conduit-test-scaffold [in] validate --keep-readme --keep-license` | PASS |  |
| 516 | `pkg:conduit-test-scaffold [in] validate --validate-git-archive` | PASS |  |
| 517 | `pkg:conduit-test-scaffold [in] AI_AGENT validate` | PASS |  |
| 518 | `pkg:conduit-test-scaffold [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 519 | `pkg:conduit-test-scaffold [in] validate -v` | PASS |  |
| 520 | `pkg:conduit-test-scaffold [in] create --dry-run` | PASS |  |
| 521 | `pkg:conduit-test-scaffold [in] update --dry-run` | PASS |  |
| 522 | `pkg:conduit-test-scaffold [in] update` | PASS |  |
| 523 | `pkg:conduit-test-scaffold [in] update -a` | PASS |  |
| 524 | `pkg:conduit-test-scaffold [in] update --group` | PASS |  |
| 525 | `pkg:conduit-test-scaffold [in] create --force` | PASS |  |
| 526 | `pkg:conduit-test-scaffold [in] AI_AGENT update` | PASS |  |
| 527 | `pkg:github-client [in] validate` | PASS |  |
| 528 | `pkg:github-client [in] validate --diff` | PASS |  |
| 529 | `pkg:github-client [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 530 | `pkg:github-client [in] validate --enforce-alignment` | PASS |  |
| 531 | `pkg:github-client [in] validate -s` | PASS |  |
| 532 | `pkg:github-client [in] validate --enforce-strict-order` | PASS |  |
| 533 | `pkg:github-client [in] validate --keep-readme --keep-license` | PASS |  |
| 534 | `pkg:github-client [in] validate --validate-git-archive` | PASS |  |
| 535 | `pkg:github-client [in] AI_AGENT validate` | PASS |  |
| 536 | `pkg:github-client [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 537 | `pkg:github-client [in] validate -v` | PASS |  |
| 538 | `pkg:github-client [in] create --dry-run` | PASS |  |
| 539 | `pkg:github-client [in] update --dry-run` | PASS |  |
| 540 | `pkg:github-client [in] update` | PASS |  |
| 541 | `pkg:github-client [in] update -a` | PASS |  |
| 542 | `pkg:github-client [in] update --group` | PASS |  |
| 543 | `pkg:github-client [in] create --force` | PASS |  |
| 544 | `pkg:github-client [in] AI_AGENT update` | PASS |  |
| 545 | `pkg:github-zero [in] validate` | PASS |  |
| 546 | `pkg:github-zero [in] validate --diff` | PASS |  |
| 547 | `pkg:github-zero [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 548 | `pkg:github-zero [in] validate --enforce-alignment` | PASS |  |
| 549 | `pkg:github-zero [in] validate -s` | PASS |  |
| 550 | `pkg:github-zero [in] validate --enforce-strict-order` | PASS |  |
| 551 | `pkg:github-zero [in] validate --keep-readme --keep-license` | PASS |  |
| 552 | `pkg:github-zero [in] validate --validate-git-archive` | PASS |  |
| 553 | `pkg:github-zero [in] AI_AGENT validate` | PASS |  |
| 554 | `pkg:github-zero [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 555 | `pkg:github-zero [in] validate -v` | PASS |  |
| 556 | `pkg:github-zero [in] create --dry-run` | PASS |  |
| 557 | `pkg:github-zero [in] update --dry-run` | PASS |  |
| 558 | `pkg:github-zero [in] update` | PASS |  |
| 559 | `pkg:github-zero [in] update -a` | PASS |  |
| 560 | `pkg:github-zero [in] update --group` | PASS |  |
| 561 | `pkg:github-zero [in] create --force` | PASS |  |
| 562 | `pkg:github-zero [in] AI_AGENT update` | PASS |  |
| 563 | `pkg:laravel-ai-parallel-tools [in] validate` | PASS |  |
| 564 | `pkg:laravel-ai-parallel-tools [in] validate --diff` | PASS |  |
| 565 | `pkg:laravel-ai-parallel-tools [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 566 | `pkg:laravel-ai-parallel-tools [in] validate --enforce-alignment` | PASS |  |
| 567 | `pkg:laravel-ai-parallel-tools [in] validate -s` | PASS |  |
| 568 | `pkg:laravel-ai-parallel-tools [in] validate --enforce-strict-order` | PASS |  |
| 569 | `pkg:laravel-ai-parallel-tools [in] validate --keep-readme --keep-license` | PASS |  |
| 570 | `pkg:laravel-ai-parallel-tools [in] validate --validate-git-archive` | PASS |  |
| 571 | `pkg:laravel-ai-parallel-tools [in] AI_AGENT validate` | PASS |  |
| 572 | `pkg:laravel-ai-parallel-tools [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 573 | `pkg:laravel-ai-parallel-tools [in] validate -v` | PASS |  |
| 574 | `pkg:laravel-ai-parallel-tools [in] create --dry-run` | PASS |  |
| 575 | `pkg:laravel-ai-parallel-tools [in] update --dry-run` | PASS |  |
| 576 | `pkg:laravel-ai-parallel-tools [in] update` | PASS |  |
| 577 | `pkg:laravel-ai-parallel-tools [in] update -a` | PASS |  |
| 578 | `pkg:laravel-ai-parallel-tools [in] update --group` | PASS |  |
| 579 | `pkg:laravel-ai-parallel-tools [in] create --force` | PASS |  |
| 580 | `pkg:laravel-ai-parallel-tools [in] AI_AGENT update` | PASS |  |
| 581 | `pkg:laravel-say-logger [in] validate` | PASS |  |
| 582 | `pkg:laravel-say-logger [in] validate --diff` | PASS |  |
| 583 | `pkg:laravel-say-logger [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 584 | `pkg:laravel-say-logger [in] validate --enforce-alignment` | PASS |  |
| 585 | `pkg:laravel-say-logger [in] validate -s` | PASS |  |
| 586 | `pkg:laravel-say-logger [in] validate --enforce-strict-order` | PASS |  |
| 587 | `pkg:laravel-say-logger [in] validate --keep-readme --keep-license` | PASS |  |
| 588 | `pkg:laravel-say-logger [in] validate --validate-git-archive` | PASS |  |
| 589 | `pkg:laravel-say-logger [in] AI_AGENT validate` | PASS |  |
| 590 | `pkg:laravel-say-logger [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 591 | `pkg:laravel-say-logger [in] validate -v` | PASS |  |
| 592 | `pkg:laravel-say-logger [in] create --dry-run` | PASS |  |
| 593 | `pkg:laravel-say-logger [in] update --dry-run` | PASS |  |
| 594 | `pkg:laravel-say-logger [in] update` | PASS |  |
| 595 | `pkg:laravel-say-logger [in] update -a` | PASS |  |
| 596 | `pkg:laravel-say-logger [in] update --group` | PASS |  |
| 597 | `pkg:laravel-say-logger [in] create --force` | PASS |  |
| 598 | `pkg:laravel-say-logger [in] AI_AGENT update` | PASS |  |
| 599 | `pkg:packagist-client [in] validate` | PASS |  |
| 600 | `pkg:packagist-client [in] validate --diff` | PASS |  |
| 601 | `pkg:packagist-client [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 602 | `pkg:packagist-client [in] validate --enforce-alignment` | PASS |  |
| 603 | `pkg:packagist-client [in] validate -s` | PASS |  |
| 604 | `pkg:packagist-client [in] validate --enforce-strict-order` | PASS |  |
| 605 | `pkg:packagist-client [in] validate --keep-readme --keep-license` | PASS |  |
| 606 | `pkg:packagist-client [in] validate --validate-git-archive` | PASS |  |
| 607 | `pkg:packagist-client [in] AI_AGENT validate` | PASS |  |
| 608 | `pkg:packagist-client [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 609 | `pkg:packagist-client [in] validate -v` | PASS |  |
| 610 | `pkg:packagist-client [in] create --dry-run` | PASS |  |
| 611 | `pkg:packagist-client [in] update --dry-run` | PASS |  |
| 612 | `pkg:packagist-client [in] update` | PASS |  |
| 613 | `pkg:packagist-client [in] update -a` | PASS |  |
| 614 | `pkg:packagist-client [in] update --group` | PASS |  |
| 615 | `pkg:packagist-client [in] create --force` | PASS |  |
| 616 | `pkg:packagist-client [in] AI_AGENT update` | PASS |  |
| 617 | `pkg:spotify-client [in] validate` | PASS |  |
| 618 | `pkg:spotify-client [in] validate --diff` | PASS |  |
| 619 | `pkg:spotify-client [in] validate --report-stale-export-ignores --diff` | PASS |  |
| 620 | `pkg:spotify-client [in] validate --enforce-alignment` | PASS |  |
| 621 | `pkg:spotify-client [in] validate -s` | PASS |  |
| 622 | `pkg:spotify-client [in] validate --enforce-strict-order` | PASS |  |
| 623 | `pkg:spotify-client [in] validate --keep-readme --keep-license` | PASS |  |
| 624 | `pkg:spotify-client [in] validate --validate-git-archive` | PASS |  |
| 625 | `pkg:spotify-client [in] AI_AGENT validate` | PASS |  |
| 626 | `pkg:spotify-client [in] AI_AGENT validate --validate-git-archive` | PASS |  |
| 627 | `pkg:spotify-client [in] validate -v` | PASS |  |
| 628 | `pkg:spotify-client [in] create --dry-run` | PASS |  |
| 629 | `pkg:spotify-client [in] update --dry-run` | PASS |  |
| 630 | `pkg:spotify-client [in] update` | PASS |  |
| 631 | `pkg:spotify-client [in] update -a` | PASS |  |
| 632 | `pkg:spotify-client [in] update --group` | PASS |  |
| 633 | `pkg:spotify-client [in] create --force` | PASS |  |
| 634 | `pkg:spotify-client [in] AI_AGENT update` | PASS |  |

## Expected-unsupported differences

### `fixture:valid [in] validate -vvv`

stderr differs

Reason: the phar's Box requirements checker prints PHP runtime details on stderr at -vvv; lpv-rs has no PHP runtime.

```diff
--- lpv stderr
+++ lpv-rs stderr
@@ -1,18 +0,0 @@
-
-Box Requirements Checker
-========================
-
-> Using PHP 8.5.12
-> PHP is using the following php.ini file:
-  /etc/php/php.ini
-
-> Checking Box requirements:
-  ✔ This application requires a PHP version matching
-"~8.2.0||~8.3.0||~8.4.0||~8.5.0||~8.6.0".
-  ✔ The package "symfony/polyfill-mbstring" requires the extension "iconv".
-  ✔ The package "symfony/service-contracts" conflicts with the extension "psr".
-  
-                                                                                
- [OK] Your system is ready to run the application.                              
-                                                                                
-
```

### `fixture:valid [in] validate --report-impact`

exit code 0 != 1; stdout differs; stderr differs

Reason: --report-impact (distribution impact report) is not ported.

```diff
--- lpv stdout
+++ lpv-rs stdout
@@ -1,10 +0,0 @@
-
-Distribution impact
-───────────────────────────────────────────
-Original archive:               0.00 MB
-Optimized archive:              0.00 MB
-Size reduction:                 19.5%
-Data saved/download:            0.00 MB
-Estimated data saved:           8.77 MB
-Estimated CO₂ emission avoided: 2.19 g CO₂
-
```

```diff
--- lpv stderr
+++ lpv-rs stderr
@@ -0,0 +1 @@
+lpv-rs: --report-impact (distribution impact report) is not ported to lpv-rs yet.
```

exit codes: lpv=0 lpv-rs=1

### `fixture:valid [in] lst <yes.txt`

exit code 0 != 1; stdout differs; stderr differs

Reason: confirming the suggested `list` command runs a command lpv-rs does not port.

```diff
--- lpv stdout
+++ lpv-rs stdout
@@ -5,30 +5,3 @@
 
  Do you want to run "list" instead?  (yes/no) [no]:
  > 
-Lean package validator 6.3.2
-
-Usage:
-  command [options] [arguments]
-
-Options:
-  -h, --help            Display help for the given command. When no command is given display help for the list command
-      --silent          Do not output any message
-  -q, --quiet           Only errors are displayed. All other output is suppressed
-  -V, --version         Display this application version
-      --ansi|--no-ansi  Force (or disable --no-ansi) ANSI output
-  -n, --no-interaction  Do not ask any interactive question
-  -v|vv|vvv, --verbose  Increase the verbosity of messages: 1 for normal output, 2 for more verbose output and 3 for debug
-
-Available commands:
-  completion       Dump the shell completion script
-  create           Create a new .gitattributes file for a project/micro-package repository
-  help             Display help for a command
-  init             Create a default .lpv file in a given project/micro-package repository
-  list             List commands
-  list-skills      List included AI skills
-  reformat         Reformat a present .gitattributes file
-  refresh          Refresh a present .lpv file
-  tree             Display the source structure of a given project/micro-package repository or it's dist package
-  update           Update an existing .gitattributes file for a project/micro-package repository
-  validate         Validate the .gitattributes file of a given project/micro-package repository
-  validate-remote  Validate the .gitattributes file of a remote GitHub repository and create or update it in a local clone
```

```diff
--- lpv stderr
+++ lpv-rs stderr
@@ -0,0 +1 @@
+lpv-rs: the "list" command of lean-package-validator is not ported to lpv-rs (yet). Supported: validate, create, update.
```

exit codes: lpv=0 lpv-rs=1

### `fixture:negated [parent] update --keep-license`

stderr differs

Reason: lpv chdir()s into the package during negated generation, so the relative write fails with a PHP engine warning naming the phar's install path; lpv-rs fails the same way (same exit code, stdout and file bytes) but prints its own warning.

```diff
--- lpv stderr
+++ lpv-rs stderr
@@ -1 +1 @@
-PHP Warning:  file_put_contents(negated/.gitattributes): Failed to open stream: No such file or directory in phar://$LPV_PACKAGES/lean-package-validator.phar/src/Gitattributes/FileRepository.php on line 97
+lpv-rs: warning: cannot write negated/.gitattributes: No such file or directory
```

### `fixture:negated [parent] create --force --keep-license`

stderr differs

Reason: lpv chdir()s into the package during negated generation, so the relative write fails with a PHP engine warning naming the phar's install path; lpv-rs fails the same way (same exit code, stdout and file bytes) but prints its own warning.

```diff
--- lpv stderr
+++ lpv-rs stderr
@@ -1 +1 @@
-PHP Warning:  file_put_contents(negated/.gitattributes): Failed to open stream: No such file or directory in phar://$LPV_PACKAGES/lean-package-validator.phar/src/Gitattributes/FileRepository.php on line 71
+lpv-rs: warning: cannot write negated/.gitattributes: No such file or directory
```

### `fixture:negated-bin [parent] update --keep-license`

stderr differs

Reason: lpv chdir()s into the package during negated generation, so the relative write fails with a PHP engine warning naming the phar's install path; lpv-rs fails the same way (same exit code, stdout and file bytes) but prints its own warning.

```diff
--- lpv stderr
+++ lpv-rs stderr
@@ -1 +1 @@
-PHP Warning:  file_put_contents(negated-bin/.gitattributes): Failed to open stream: No such file or directory in phar://$LPV_PACKAGES/lean-package-validator.phar/src/Gitattributes/FileRepository.php on line 97
+lpv-rs: warning: cannot write negated-bin/.gitattributes: No such file or directory
```

### `fixture:negated-bin [parent] create --force --keep-license`

stderr differs

Reason: lpv chdir()s into the package during negated generation, so the relative write fails with a PHP engine warning naming the phar's install path; lpv-rs fails the same way (same exit code, stdout and file bytes) but prints its own warning.

```diff
--- lpv stderr
+++ lpv-rs stderr
@@ -1 +1 @@
-PHP Warning:  file_put_contents(negated-bin/.gitattributes): Failed to open stream: No such file or directory in phar://$LPV_PACKAGES/lean-package-validator.phar/src/Gitattributes/FileRepository.php on line 71
+lpv-rs: warning: cannot write negated-bin/.gitattributes: No such file or directory
```

