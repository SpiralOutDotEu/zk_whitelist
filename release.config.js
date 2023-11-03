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
        {"path": "target/release/zk_whitelist", "label": "Linux binary"},
        {"path": "target/release/zk_whitelist.exe", "label": "Windows binary"},
        {"path": "target/release/zk_whitelist.dmg", "label": "macOS binary"}
      ]
    }],
    ["@semantic-release/git", {
      "assets": ["CHANGELOG.md", "Cargo.toml", "Cargo.lock"],
      "message": "chore(release): ${nextRelease.version} [skip ci]\n\n${nextRelease.notes}"
    }],
  ]
}
