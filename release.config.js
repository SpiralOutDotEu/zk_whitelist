module.exports = {
  branches: ["master"],
  plugins: [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    ["@semantic-release/exec", {
      "prepareCmd": "./update_version.sh ${nextRelease.version}"
    }],
    ["@semantic-release/changelog", {
      "changelogFile": "CHANGELOG.md",
    }],
    ["@semantic-release/github", {
      "assets": [
        {"path": "zk_whitelist_linux.tar.gz", "label": "Linux binary"},
        {"path": "zk_whitelist_windows.zip", "label": "Windows binary"},
        {"path": "zk_whitelist_macos.tar.gz", "label": "macOS binary"}
      ]
    }],
    ["@semantic-release/git", {
      "assets": ["CHANGELOG.md", "Cargo.toml", "Cargo.lock"],
      "message": "chore(release): ${nextRelease.version} [skip ci]\n\n${nextRelease.notes}"
    }],
  ]
}
