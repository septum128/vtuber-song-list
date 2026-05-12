function readPackage(pkg, context) {
  // Allow build scripts for dependencies that require native compilation
  if (
    pkg.name === "@parcel/watcher" ||
    pkg.name === "sharp" ||
    pkg.name === "unrs-resolver"
  ) {
    pkg.pnpm = pkg.pnpm || {};
    pkg.pnpm.allowBuild = true;
  }
  return pkg;
}

module.exports = {
  hooks: {
    readPackage,
  },
};
