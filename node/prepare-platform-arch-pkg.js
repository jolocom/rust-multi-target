/**
 * This script will create a platform specific package
 * such as '@jolocom/native-core-node-linux-x64' using the
 * parent directory's package.json and build output
 */

const path = require('path')
const fs = require('fs-extra')

const baseDir = path.resolve(__dirname)
process.chdir(baseDir)

const tmplPkgJsonPath = path.join(baseDir, 'package.json')
const outDir = path.resolve(__dirname, 'platform-arch-pkg')
const outPkgJsonPath = path.join(outDir, 'package.json')

const pkgJson = JSON.parse(fs.readFileSync(tmplPkgJsonPath).toString())

const platformArch = `${process.platform}-${process.arch}`
pkgJson.cpu = [process.arch]
pkgJson.os = [process.platform]
pkgJson.name += `-${platformArch}`
pkgJson.description += ` [${platformArch}]`

console.log(
  '\n\n\n' +
  'Generating platform specific package: ' + pkgJson.name
)

// Remove properties we don't need in the package.json file
// for the native packages
delete pkgJson.optionalDependencies
delete pkgJson.scripts

// add the 'index.node' binary file to the packaged files
pkgJson.files.push('native/index.node', 'README', 'LICENSE', 'CONTRIBUTING')

// clear out the output directory
fs.removeSync(outDir)
fs.mkdirpSync(outDir)

// write out platform-arch-pkg/package.json
fs.writeFileSync(outPkgJsonPath, JSON.stringify(pkgJson, null, 2))

// and copy over all files necessary for packaging
pkgJson.files.forEach(p => {
  try {
    console.log('copying ' + p)
    const outPath = path.join(outDir, p)
    fs.copySync(p, outPath)
  } catch (err) {
    console.error('could not copy ', p)
  }
})

console.log(
  'Generated at ' + outDir +
  '\n\n\n'
)
