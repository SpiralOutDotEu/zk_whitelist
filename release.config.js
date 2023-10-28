module.exports = {
    branches: ["main"],
    plugins: [
      "@semantic-release/commit-analyzer",
      "@semantic-release/release-notes-generator",
      ["@semantic-release/changelog", {
        "changelogFile": "CHANGELOG.md",
      }],
      ["@semantic-release/github", {
        "assets": "target/release/zk_whitelist"
      }],
      ["@semantic-release/exec", {
        "prepareCmd": "./update_version.sh ${nextRelease.version}"
      }],
      ["@semantic-release/git", {
        "assets": ["CHANGELOG.md", "Cargo.toml"],
        "message": "chore(release): ${nextRelease.version} [skip ci]\n\n${nextRelease.notes}"
      }],
    ]
  }
